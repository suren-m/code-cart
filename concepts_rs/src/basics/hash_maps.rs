use rand::prelude::*;
use std::{collections::HashMap, hash::Hash};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Video {
    id: Uuid,
}

impl Video {

    fn new() -> Self {
        Video { id: Uuid::new_v4() }
    }
}

fn get_video_views(vid_count: u32) -> HashMap<Video, u32> {
    //write test here
    let mut video_views = HashMap::new();
    let mut rng = rand::thread_rng();
    for _ in 0..vid_count {
        let video = Video::new();
        let view_count: u32 = rng.gen_range(1..1000);
        video_views.insert(video, view_count);
    }
    video_views
}

fn print(video_views: &HashMap<Video, u32>) {
    // for (k, v) in video_views {
    //     println!("k: {} v: {}", k.id, v);
    // }
    println!("{:#?}", video_views);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let video_views = get_video_views(20);
        print(&video_views);


    }
}
