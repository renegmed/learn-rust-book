use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {   // default implementation
        String::from("(Read more...)")
    }

}
// ---- Iplementing a Trait on a Type
pub struct NewsArticle {
    pub headline: String, 
    pub location: String, 
    pub author: String, 
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
    fn summarize_author(&self) -> String {
        format!("author summary: {}", self.author)        
    }

    // fn new(&self) -> Self {
    //     Self {headline: "", location: "", author: "", content: ""}   
    // }
}

// impl<T: Display> ToString for NewsArticle {
//     fn to_string(&self) -> String {
//         format!("Headline: {}\n....written by {}",self.headline, self.author)
//     }
// }
 

pub struct Tweet {
    pub username: String, 
    pub content: String, 
    pub reply: bool, 
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        //format!("{}: {}", self.username, self.content)
        format!("(Read more FROM {}", self.summarize_author())
    }

    /*
    
    NOTE: it isn’t possible to call the default implementation from 
          an overriding implementation of that same method.
        e.g. 
          format!("{}, {}", super.summarize(), self.summarize_author)
          
    */

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
// --- traits as parameter 
pub fn notify(item: &impl Summary) {
    println!("Breaking news! traits as parameter {}", item.summarize());
}

// --- trait bound syntax
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! trait bound syntax {}", item.summarize());
}

// --- trait bound syntax longer form
pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
    println!("Multiple trait parameters long form\n\t{} \n\t{}", item1.summarize(), item2.summarize());
}

// --- trait bound syntax shorter form
pub fn notify4<T: Summary, U: Summary>(item1: &T, item2: &U) {
    println!("Multiple trait parameters short form\n\t{} \n\t{}", 
        item1.summarize(), item2.summarize());
}

// --- specifying multiple trait bounds with the +syntax 
pub fn notify5<T: Summary + std::fmt::Display>(item: &T) {
    println!("Specify multiple trait bounds with +syntax \n\t{}", item.summarize());
} 

pub fn notify6(item: &(impl Summary + std::fmt::Display)) {
    println!("Another specify multiple trait bounds with +syntax \n\t{}", item.summarize());
} 

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {             
            largest = item;
        }
    }
    largest
}

// --- Clearer trait bounds with where clauses 
/*

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

    use instead

fn some_function<T, U>(t: &T, u: &U) -> i32 
    where T: Display + Clone, 
          U: Clone + Debug    
{
   ... 
}

*/

// ---- returning types that implement traits 
fn returns_summarizable() -> impl Summary {
    Tweet{
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, AS you probable already know, people"
        ),
        reply: true, 
        retweet: false,
    }
}

/*
THIS WON'T WORK

Returning either a NewsArticle or a Tweet isn’t allowed due to restrictions around how 
the impl Trait syntax is implemented in the compiler.

You can only use impl Trait if you’re returning a single type. For example, this code 
that returns either a NewsArticle or a Tweet with the return type specified as 
impl Summary wouldn’t work:


fn returns_summarizable2(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
*/

// -------- using trait bounds to conditionally implement methods 
// Conditionally implement methods on a generic type depending on trait bounds
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }    
    }
}

fn main() {
    
    // ----------------------------

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false, 
        retweet: false,
    };

    println!("1 new tweet summary: {}", tweet.summarize());
    
    // using default implementation when no impl 
    let article = NewsArticle {
        headline: String::from("Peguins win the Stanley Cup Championship!"),
        location: String::from("Pottsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Peguins once again are the best \
            bockey team in the NHL."
        ),
    };

    println!("New article available! {}", article.summarize()); // no impl on method, use default

    // --- traits as parameter 
    
    notify(&article); 

    // --- trait bound syntax
    notify2(&article); 
    
    // --- trait bound syntax longer form
    notify3(&article, &tweet); 

    // --- trait bound syntax short form
    notify4(&article, &tweet); 

    // --- specifying multiple trait bounds with the +syntax 
    notify5(&article); 
    // notify6(&article); 
 

    let number_list = vec![34, 58, 25, 100, 65,120];
    
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // ---- returning types that implement traits 
    let tweet2 = returns_summarizable();
    println!("Returned summary tweet summarize():\n\t{}", tweet2.summarize());

    // -------- using trait bounds to conditionally implement methods 
    // Conditionally implement methods on a generic type depending on trait bounds
    let pair = Pair::new(20, 35);
    pair.cmp_display();

}
