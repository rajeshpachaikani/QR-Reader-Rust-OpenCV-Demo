/*
    Author: Rajesh Pachaikani
    Date: 2020-06-10

    Description:
        This is a simple Rust program to illustrate how to read QR Code using opencv.

*/
use opencv::{
    Result,
    prelude::*,
    objdetect,
    imgproc,
    highgui,
    types,
    videoio,
    core,
};


fn main() -> Result<()> {
    let mut qr_detector = objdetect::QRCodeDetector::default()?;
    let mut res = types::VectorOfPoint::new();
    let mut camera = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut img = Mat::default();
    let mut recqr = Mat::default();
    loop{
        camera.read(&mut img)?;
        let ret = qr_detector.detect_and_decode(&img, &mut res, &mut recqr)?;
        let s = String::from_utf8_lossy(&ret);
        println!("{:?}", res);
        println!("{:?}", s);
        highgui::named_window("QR Code", highgui::WINDOW_NORMAL)?;
        if recqr.size()?.width > 0{
            highgui::imshow("QR Code", &recqr)?;
        }
        if res.len()>0 {
            imgproc::polylines(
                &mut img, 
                &res, 
                true, 
                core::Scalar::new(0f64,255f64,0f64,0f64), 
                1,
                1,
                0)?;
        }

        highgui::imshow("Frame", &img)?;
        let key = highgui::wait_key(1)?;
        if key == 'q' as i32 {
            break;
        }
    }
    Ok(())
}
