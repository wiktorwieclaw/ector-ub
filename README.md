# Example how to trigger undefined behavior in safe Rust using `ector` 0.2
`cargo miri run` output:
```
error: Undefined Behavior: trying to retag from <17366> for SharedReadOnly permission at alloc2[0xa0], but that tag does not exist in the borrow stack for this location
   --> C:\Users\Vixu\.cargo\registry\src\index.crates.io-6f17d22bba15001f\ector-0.4.0\src\actor.rs:170:9
    |
170 |         self.reply_to.send(value).await
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^
    |         |
    |         trying to retag from <17366> for SharedReadOnly permission at alloc2[0xa0], but that tag does not exist in the borrow stack for this location
    |         this error occurs as part of retag at alloc2[0xa0..0xb0]
    |
    = help: this indicates a potential bug in the program: it performed an invalid operation, but the Stacked Borrows rules it violated are still experimental
    = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
help: <17366> was created by a SharedReadOnly retag at offsets [0xa0..0xb0]
   --> src\main.rs:13:89
    |
13  |     let _ = embassy_time::with_timeout(Duration::from_micros(1), addr.request("Hello")).await;
    |                                                                                         ^^^^^
help: <17366> was later invalidated at offsets [0x30..0x220] by a write access
   --> src\main.rs:24:24
    |
24  |     join(server, test).await;
    |                        ^^^^^
    = note: BACKTRACE (of the first span):
    = note: inside closure at C:\Users\Vixu\.cargo\registry\src\index.crates.io-6f17d22bba15001f\ector-0.4.0\src\actor.rs:170:9: 170:34
note: inside closure
   --> src\main.rs:46:27
    |
46  |             motd.reply(m).await;
    |                           ^^^^^
    = note: inside closure at C:\Users\Vixu\.cargo\registry\src\index.crates.io-6f17d22bba15001f\ector-0.4.0\src\actor.rs:231:43: 231:48
    = note: inside `embassy_futures::join::MaybeDone::<[async fn body@ector::ActorContext<Server>::mount::{closure#0}]>::poll` at C:\Users\Vixu\.cargo\registry\src\index.crates.io-6f17d22bba15001f\embassy-futures-0.1.0\src\join.rs:24:40: 24:83
    = note: inside `<embassy_futures::join::Join<[async fn body@ector::ActorContext<Server>::mount::{closure#0}], [async fn body@src\main.rs:12:74: 14:2]> as std::future::Future>::poll` at C:\Users\Vixu\.cargo\registry\src\index.crates.io-6f17d22bba15001f\embassy-futures-0.1.0\src\join.rs:95:33: 95:87
note: inside closure
   --> src\main.rs:24:24
    |
24  |     join(server, test).await;
    |                        ^^^^^
    = note: inside `embassy_executor::raw::TaskStorage::<[async fn body@src\main.rs:16:1: 16:26]>::poll` at C:\Users\Vixu\.cargo\git\checkouts\embassy-9312dcb0ed774b29\3e730aa\embassy-executor\src\raw\mod.rs:164:15: 164:35
    = note: inside closure at C:\Users\Vixu\.cargo\git\checkouts\embassy-9312dcb0ed774b29\3e730aa\embassy-executor\src\raw\mod.rs:431:17: 431:57
    = note: inside `embassy_executor::raw::run_queue::RunQueue::dequeue_all::<[closure@embassy_executor::raw::SyncExecutor::poll::{closure#1}]>` at C:\Users\Vixu\.cargo\git\checkouts\embassy-9312dcb0ed774b29\3e730aa\embassy-executor\src\raw\run_queue.rs:85:13: 85:26
    = note: inside `embassy_executor::raw::SyncExecutor::poll` at C:\Users\Vixu\.cargo\git\checkouts\embassy-9312dcb0ed774b29\3e730aa\embassy-executor\src\raw\mod.rs:411:13: 439:15
    = note: inside `embassy_executor::raw::Executor::poll` at C:\Users\Vixu\.cargo\git\checkouts\embassy-9312dcb0ed774b29\3e730aa\embassy-executor\src\raw\mod.rs:536:9: 536:26
    = note: inside `embassy_executor::Executor::run::<[closure@src\main.rs:16:1: 16:26]>` at C:\Users\Vixu\.cargo\git\checkouts\embassy-9312dcb0ed774b29\3e730aa\embassy-executor\src\arch\std.rs:67:26: 67:43
note: inside `main`
   --> src\main.rs:16:1
    |
16  | #[embassy_executor::main]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: this error originates in the attribute macro `embassy_executor::main` (in Nightly builds, run with -Z macro-backtrace for more info)

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to previous error; 1 warning emitted

error: process didn't exit successfully: `C:\Users\Vixu\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\cargo-miri.exe runner target\miri\x86_64-pc-windows-msvc\debug\ector-use-after-free.exe` (exit code: 1)
```