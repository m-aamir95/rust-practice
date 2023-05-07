struct ToDownload{
    _url: String,
    datasetname: String,
    _description: String,
    resource_location : String
}

fn main() {
    
    // Define the file to download
    let url = "https://gist.githubusercontent.com/curran/a08a1080b88344b0c8a7/raw/0e7a9b0a5d22642a06d3d5b9bcbad9890c8ee534/iris.csv";
    let to_download = ToDownload{ _url :url.to_string(),
                                              datasetname : String::from("Iris-setosa"),
                                              _description : String::from("Iris setosa dataset github link"),
                                              resource_location: String::from("Github Gists")
                                             };

    println!("\n\nDownloading {} via {}", to_download.datasetname, to_download.resource_location);


    let req = reqwest::blocking::Client::new();

    let mut response =  req.get(to_download._url).send().unwrap();

    let mut output_file = std::fs::File::create(to_download.datasetname+".csv").unwrap();

    response.copy_to(&mut output_file).unwrap();
}
