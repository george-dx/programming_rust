use std::sync::{mpsc, Mutex};
use std::{fs, thread};

// fn start_file_reader_thread(
//     documents: Vec<PathBuf>,
// ) -> (mpsc::Receiver<String>, thread::JoinHandle<io::Result<()>>) {
//     let (sender, receiver) = mpsc::channel();
//     let handle = thread::spawn(move || {
//         println!("Blabla");
//     });
//     (receiver, handle)
// }

// fn start_file_indexing_thread(
//     texts: mpsc::Receiver<String>,
// ) -> (mpsc::Receiver<InMemoryIndex>, thread::JoinHandle<()>) {
//     let (sender, receiver) = mpsc::channel();

//     let handle = thread::spawn(move || {
//         for (doc_id, text) in texts.into_iter().enumerate() {
//             let index = InMemoryIndex::from_single_document(doc_id, text);
//             if sender.send(index).is_err() {
//                 break;
//             }
//         }
//     });
// }

type PlayerId = u32;

const GAME_SIZE: usize = 8;

type WaitingList = Vec<PlayerId>;

struct FernEmpireApp {
    waiting_list: Mutex<WaitingList>,
}

impl FernEmpireApp {
    fn join_waiting_list(&self, player: PLayerId) {
        let mut guard = self.waiting_list.lock().unwrap();

        guard.push(player);
        if guard.len() == GAME_SIZE {
            let players = guard.split_off(0);
            self.start_game(players);
        }
    }
}

fn main() {}
