#[derive(Debug)]
enum Gender {
  Unspecified = 0,
  Female = 1,
  Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
  id: UserId,
  name: String,
  gender: Gender,
}

#[derive(Debug)]
struct Topic {
  id: TopicId,
  name: String,
  owner: UserId,
}

#[derive(Debug)]
enum Event {
  Join((UserId, TopicId)),
  Leave((UserId, TopicId)),
  Message((UserId, TopicId, String)),
}

// 斐波那契数列
// fn fib_loop(n: u8) {
//   let mut a = 1;
//   let mut b = 1;
//   let mut i = 2u8;

//   loop {
//     let c = a + b;
//     a = b;
//     b = c;
//     i += 1;

//     println!("next val is {}", b);

//     if i >= n {
//       break;
//     }
//   }
// }

// fn fib_while(n: u8) {
//   let (mut a, mut b, mut i): (u8, u8, u8) = (1, 1, 2);

//   while i < n {
//     let c = a + b;
//     a = b;
//     b = c;
//     i += 1;

//     println!("next val is {}", b);
//   }
// }

// fn fib_for(n: u8) {
//   let (mut a, mut b) = (1, 1);

//   for _i in 2..n {
//     let c = a + b;
//     a = b;
//     b = c;
//     println!("next val is {}", b);
//   }
// }

// 抽取复用结构
fn calc_fib(n: u8, f: fn(u8, &mut Vec<u8>)) {
  let mut result = vec![1, 1];
  f(n, &mut result);
  for val in result {
    println!("next val is {}", val);
  }
}

fn fib_loop(n: u8) {
  fn loop_fib(n: u8, result: &mut Vec<u8>) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;
    loop {
      let c = a + b;
      result.push(c);
      a = b;
      b = c;
      i += 1;

      if i >= n {
        break;
      }
    }
  }
  calc_fib(n, loop_fib);
}

fn fib_while(n: u8) {
  fn while_fib(n: u8, result: &mut Vec<u8>) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;
    while i < n {
      let c = a + b;
      result.push(c);
      a = b;
      b = c;
      i += 1;
    }
  }
  calc_fib(n, while_fib);
}

fn fib_for(n: u8) {
  fn for_fib(n: u8, result: &mut Vec<u8>) {
    let mut a = 1;
    let mut b = 1;
    for i in 2..n {
      let c = a + b;
      result.push(c);
      a = b;
      b = c;
    }
  }
  calc_fib(n, for_fib);
}

fn process_event(event: &Event) {
  match event {
    Event::Join((uid, _tid)) => println!("user {:?} joined", uid),
    Event::Leave((uid, tid)) => println!("user {:?} left {:?}", uid, tid),
    Event::Message((_, _, msg)) => println!("broadcast: {}", msg),
  }
}

fn main() {
  let n = 5;
  let alice = User {
    id: UserId(1),
    name: "Alice".into(),
    gender: Gender::Female,
  };
  let bob = User {
    id: UserId(2),
    name: "Bob".into(),
    gender: Gender::Male,
  };

  let topic = Topic {
    id: TopicId(1),
    name: "rust".into(),
    owner: UserId(1),
  };
  let event1 = Event::Join((alice.id, topic.id));
  let event2 = Event::Leave((bob.id, topic.id));
  let event3 = Event::Message((alice.id, topic.id, "Hello world!".into()));

  fib_loop(n);
  fib_while(n);
  fib_for(n);
  // process_event(&event1);
  // process_event(&event2);
  // process_event(&event3);
}
