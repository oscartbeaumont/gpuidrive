use gpui::{App, Application, Context, Entity, EventEmitter, prelude::*};

struct Counter {
    count: usize,
}

struct Change {
    increment: usize,
}

impl EventEmitter<Change> for Counter {}

fn main() {
    Application::new().run(|cx: &mut App| {
        println!("START");

        let counter: Entity<Counter> = cx.new(|_cx| Counter { count: 0 });
        let subscriber = cx.new(|cx: &mut Context<Counter>| {
            println!("A");
            cx.subscribe(&counter, |subscriber, _emitter, event, _cx| {
                println!("B");
                subscriber.count += event.increment * 2;
            })
            .detach();

            Counter {
                count: counter.read(cx).count * 2,
            }
        });

        counter.update(cx, |counter, cx| {
            counter.count += 2;
            println!("C");
            cx.notify();
            println!("D");
            cx.emit(Change { increment: 2 });
        });

        assert_eq!(subscriber.read(cx).count, 4);
        println!("COMPLETE");
    });
}
