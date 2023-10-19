use clipboard_files;
use copypasta::{ClipboardContext, ClipboardProvider};
use napi::Result;

// use std::{
//   path::PathBuf,
//   sync::mpsc::{self, Sender},
//   thread::spawn,
// };

// use napi::{
//   bindgen_prelude::*,
//   threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
// };

// enum ClipBoardContent {
//   File(Vec<PathBuf>),
//   Text(String),
// }

#[napi(object)]
pub struct ClipBoardContentJson {
  #[napi(ts_type = "'file' | 'text'")]
  pub r#type: String,
  pub content: Vec<String>,
}

// struct Handler {
//   pub tx: Sender<ClipBoardContent>,
//   pub ctx: WindowsClipboardContext,
// }

// impl ClipboardHandler for Handler {
//   fn on_clipboard_change(&mut self) -> CallbackResult {
//     let files = clipboard_files::read();
//     match files {
//       Ok(f) => {
//         println!("{:#?}", f);
//         self.tx.send(ClipBoardContent::File(f)).unwrap();
//       }
//       Err(err1) => {
//         println!("{:#?}", err1);
//         let content = self.ctx.get_contents();
//         match content {
//           Ok(text) => {
//             self.tx.send(ClipBoardContent::Text(text)).unwrap();
//           }
//           Err(err) => {
//             println!("{:#?}", err);
//             // self.tx.send(None).unwrap();
//           }
//         }
//       }
//     }
//     CallbackResult::Next
//   }

//   fn on_clipboard_error(&mut self, error: std::io::Error) -> CallbackResult {
//     println!("{:#?}", error);
//     CallbackResult::Next
//   }
// }

// #[napi(ts_args_type = "callback: (content: {type:'file'|'text',content:string[]}) => void")]
// pub fn on_clipboard_change(callback: JsFunction) {
//   let jsfn: ThreadsafeFunction<ClipBoardContent, ErrorStrategy::Fatal> = callback
//     .create_threadsafe_function(0, |ctx| match ctx.value {
//       ClipBoardContent::File(f) => Ok(vec![ClipBoardContentJson {
//         r#type: "file".to_string(),
//         content: f
//           .into_iter()
//           .map(|c| c.to_str().unwrap().to_string())
//           .collect::<Vec<String>>(),
//       }]),
//       ClipBoardContent::Text(t) => Ok(vec![ClipBoardContentJson {
//         r#type: "text".to_string(),
//         content: vec![t],
//       }]),
//     })
//     .unwrap();

//   let (tx, rx) = mpsc::channel();
//   let ctx = ClipboardContext::new().unwrap();

//   spawn(|| {
//     let _ = Master::new(Handler { tx, ctx }).run();
//   });
//   spawn(move || {
//     for c in rx {
//       jsfn.call(c, ThreadsafeFunctionCallMode::Blocking);
//     }
//   });
// }

// 获取剪切板文件或者文本
#[napi]
pub fn get_clipboard_content() -> Result<Option<ClipBoardContentJson>> {
  let files = clipboard_files::read();
  let mut ctx = ClipboardContext::new().unwrap();
  match files {
    Ok(f) => Ok(Some(ClipBoardContentJson {
      r#type: "file".to_string(),
      content: f
        .into_iter()
        .map(|c| c.to_str().unwrap().to_string())
        .collect::<Vec<String>>(),
    })),
    Err(_) => {
      let content = ctx.get_contents();
      match content {
        Ok(text) => Ok(Some(ClipBoardContentJson {
          r#type: "text".to_string(),
          content: vec![text],
        })),
        Err(_) => Ok(None),
      }
    }
  }
}
