// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver};
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::SyncSender;

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

#[derive(Debug)]
pub enum Error {
    TicketInsertionError,
    TicketGetError,
}

impl TicketStoreClient {
    fn new(sender: SyncSender<Command>) -> Self {
        Self {
            sender,
        }
    }
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, Error> {
        let (sender, receiver) = sync_channel(0);

        let sent = self.sender.send(Command::Insert {
            draft,
            response_channel: sender,
        });

        match sent {
            Ok(_) => {
                Ok(receiver.recv().unwrap())
            },
            Err(_) => {
                Err(Error::TicketInsertionError)
            },
        }

    }
    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, Error> {
        let (sender, receiver) = sync_channel(0);

        let sent = self.sender.send(Command::Get {
            id,
            response_channel: sender,
        });

        match sent {
            Ok(_) => {
                let result = receiver.recv().unwrap();
                Ok(result)
            }
            Err(_) => {
                return Err(Error::TicketGetError);
            }
        }
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient::new(sender)
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);

                match ticket {
                    Some(ticket) => {
                        let ticket = ticket.clone();
                        let _ = response_channel.send(Some(ticket));
                    },
                    None => {
                        let _ = response_channel.send(None);
                    }
                }
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
