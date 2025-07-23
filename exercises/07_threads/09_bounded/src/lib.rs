// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{channel, sync_channel, Receiver, Sender, SyncSender, TrySendError};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, TrySendError<&str>> {
        let (sender, receiver) = sync_channel(10);
        let cmd = Command::Insert {
            draft,
            response_channel: sender,
        };
        let res = self.sender.try_send(cmd);
        match res {
            Ok(_) => {
                let id = receiver.recv().expect("msg");
                Ok(id)
            }
            Err(_) => {
                return Err(TrySendError::Full("msg"));
            }
        }
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, TrySendError<&str>> {
        let (sender, receiver) = sync_channel(10);
        let cmd = Command::Get {
            id,
            response_channel: sender,
        };
        let res = self.sender.try_send(cmd);
        match res {
            Ok(_) => {
                let get_res = receiver.recv().expect("msg");
                Ok(get_res)
            }
            Err(_) => {
                return Err(TrySendError::Full("msg"));
            }
        }
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
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

pub fn server(receiver: Receiver<Command>) {
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
