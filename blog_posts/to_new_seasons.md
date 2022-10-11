---
title: To New Seasons
date: April 9, 2022
tags: new season, next gen, rust, blog
series: blog
---

# To New Seasons

The site is finally live! This is my personal blog/thoughts collector. I wont ever write anything really personal here, as this is still the internet, but this will be more to talk about my thoughts on all things Tech around the world. The website will be in a beta state for a long time to come; I have a lot of ideas for the site, and it will take time to bring them together. It's like an evolving code project to show off my knowledge and learn.

The core of the site was already in place before I started designing too. I was really getting into backend server markdown parsers, and I started developing different backend rendered web sites around this concept. I have written many different builds in JS, TS, Nim, Goland and Rust. And I have bounced back and forth between tons of builds and frameworks and ideas... I have an issue with settling 😛.

Rough idea of what I am looking for in a project language:

- Fast HTTP response times
- Good community support
- Well developed packages that support the project
- Semi-quick development
- Fun development 🎉

Rust is a beautiful language with some amazing concepts! Rust just makes you feel smart haha; it does things 'right' and forces you to aswell... it just can take a long time to develop in it though... And when learning new concepts in Rust you are left trying to wrap your mind around these huge boilerplate concepts and large complex types. Good thing I have an infinite amount of time on my hands haha! Honestly though, Rust is just FUN to write in!

Lets look at the JS/TS world real quick, which I hate the state of it right now. Give it a couple more years and it will be in a perfect state. Nodejs should probably just die at this point, and I am only slightly joking. Node is just missing a lot when it comes to a user friendly modern JS/TS experience; go try and use top level async/await with TypeScript on a Node project... good luck! And then Deno offers great modern tooling, but performance is still behind and packages are minimal. Deno has a great potential but is just too young; they are having a hard time convincing people to move over from Node and loose 95% of the mature packages they use everyday. And then you have Bun, which is super fast but again very very early and just not ready yet. Give it a couple years and I think Deno and Bun take off. Node will probably be around forever just to maintain legacy code; but a lot of developers have jumped ship to Golang/Rust already.

Then there is Golang. Probably my second most written in language... but it's boring it me lol. RIP GOOGLE! Just joking, Golang is a great language that can take you very very far! I have written a lot of projects in Go and seem to enjoy my time. It does have fast development time which is awesome! It is just a little clunky and feels like I am slapping gum on a project and calling it a day. Again... Rust just does things right! But I am sure I will dive into that more in a latter post.

So here is a little snippet of a Axum Rust HTTP server! I am using Axum for this project! 🚀

```rs
use axum::{
    http::{Response, StatusCode},
    routing::{get, get_service},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new();

    let addr = SocketAddr::from((
        [0, 0, 0, 0], 80,
    ));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

So what is next?! Well hopefully I keep developing this site and learning new things to implement. And I hope the content is good and enjoyable!

### Eric Christensen
