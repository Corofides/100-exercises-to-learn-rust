use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

#[derive(Clone)]
// TODO: flesh out the client implementation.
pub struct TicketStoreClient {
    sender: Sender<Command>,
}

impl TicketStoreClient {

    pub fn new(sender: Sender<Command>) -> Self {
        Self {
            sender,
        }
    }
    // Feel free to panic on all errors, for simplicity.
    pub fn insert(&self, draft: TicketDraft) -> TicketId {

        let (sender, receiver) = std::sync::mpsc::channel();

        let command = Command::Insert {
            draft: draft.clone(),
            response_channel: sender,
        };

        let result = self.sender
            .send(command)
            .unwrap();

        let ticket_id: TicketId = receiver
            .recv()
            .expect("No response received!");

        ticket_id

    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> {

        let (sender, receiver) = std::sync::mpsc::channel();

        let command = Command::Get {
            id,
            response_channel: sender,
        };

        let result = self.sender
            .send(command)
            .unwrap();

        let ticket: Option<Ticket> = receiver
            .recv()
            .expect("No response.received!");

        ticket

    }
}

pub fn launch() -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    
    TicketStoreClient::new(sender)
    // std::thread::spawn(move || client(sender));
    //todo!()
}

// No longer public! This becomes an internal detail of the library now.
enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
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
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
