use image::{GenericImageView, Pixel};

fn main() {
    println!("enter the file path: ");
    let mut filepath = String::new();
    let _userinput = std::io::stdin().read_line(&mut filepath).unwrap();
    let img = image::open(filepath.replace("\r\n", "")).unwrap();
    let mut art: String = String::new();
    let mut column: String = String::new();
    let ascii_chars: String = " `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@".to_string();
    let resizedimg = img.resize(img.width() / 6, img.height() / 6, image::imageops::FilterType::Nearest);
    for h in 0..resizedimg.height(){
        for w in 0..resizedimg.width(){
            let pixel = resizedimg.get_pixel(w, h).to_rgb();
            let greyscale = (0.2990 * (pixel[0] as f32 / 255.0)) + (0.5870 * pixel[1] as f32 / 255.0) + (0.1140 * pixel[2] as f32 / 255.0);
            column = format!("{}{}", column, ascii_chars.as_bytes()[(greyscale * ascii_chars.len() as f32 - 1.0) as usize] as char);       
        }
        art = format!("{}{}\n", art, column);
        column = String::new();
    }
    println!("{}", art);
    println!("press enter to exit");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
