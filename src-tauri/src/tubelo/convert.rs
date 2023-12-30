use rustube::{ Id, VideoFetcher, Stream, Video };

pub struct Downloader {
    pub url: String,
}

impl Downloader {

    pub fn init(url: String) -> Self {
        Self {
            url: url,
        }
    }

    pub async fn mp3(&self) {
        let id = Id::from_raw(&self.url).unwrap();
        let descrambler = VideoFetcher::from_id(id.into_owned())
        .unwrap().fetch().await.unwrap();

        let title = &descrambler.video_details().title;
        let path = format!("{}.mp3", title);

        let vdo = &descrambler.descramble().unwrap();
        let stream = Video::best_audio(vdo).unwrap();
        
        println!("Downloading"); 
        Stream::download_to(stream, path).await.unwrap();
        println!("The audio has been downloaded");
    }

    pub async fn mp4(&self) {
        let id = Id::from_raw(&self.url).unwrap();
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