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
        let file_extension = "mp3";
        let id = Id::from_raw(&self.url).unwrap();
        let descrambler = VideoFetcher::from_id(id.into_owned())
        .unwrap().fetch().await.unwrap();

        let title = &descrambler.video_details().title;
        let path = format!("{}.{}", title,file_extension);
        println!("path: {}", path);
        let vdo = &descrambler.descramble().unwrap();
        let stream = Video::best_audio(vdo).unwrap();
        
        println!("Downloading"); 
        Stream::download_to(stream, path).await.unwrap();
        println!("The audio has been downloaded");
    }

    pub async fn mp4(&self) {
        let file_extension = "mp4";
        let id = Id::from_raw(&self.url).unwrap();
        let descrambler = VideoFetcher::from_id(id.into_owned())
        .unwrap().fetch().await.unwrap();

        let title = &descrambler.video_details().title;
        let path = format!("{}.{}", title, file_extension);

        let vdo = &descrambler.descramble().unwrap();
        let stream = Video::best_quality(vdo).unwrap();
        
        println!("Downloading"); 
        Stream::download_to(stream, path).await.unwrap();
        println!("The video has been downloaded");
    }
}