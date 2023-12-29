use rustube::{Id, VideoFetcher, Stream, Video };

pub struct Download;

impl Download {

    pub async fn mp3(&self) {
       
    }

    pub async fn mp4(&self, url: &str) {
        let id = Id::from_raw(url).unwrap();
        let descrambler = VideoFetcher::from_id(id.into_owned())
        .unwrap().fetch().await.unwrap();

        let title = &descrambler.video_details().title;
        let path = format!("{}.mp4", title);

        let vdo = &descrambler.descramble().unwrap();
        let stream = Video::best_quality(vdo).unwrap();
        
        println!("Downloading");
        Stream::download_to(stream, path).await.unwrap();
        println!("The video has been downloaded");
    }
}