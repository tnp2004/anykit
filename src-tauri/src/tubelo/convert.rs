pub struct Download;

impl Download {

    pub async fn mp3(&self) {
       
    }

    pub async fn mp4(&self, url: &str) {
        println!("downloaded video to {:?}", rustube::download_best_quality(url).await.unwrap());
    }
}