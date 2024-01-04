use bytes::{Buf, BufMut};
use image::GenericImageView;
fn icns(icon: &image::DynamicImage, icns: &str) -> anyhow::Result<()> {
    // Load an icon family from an ICNS file.
    let file = std::fs::read(icns)?;

    let file = file.reader();
    let (width, height) = icns::IconFamily::read(file)?
        .elements
        .iter()
        .filter_map(|i| i.decode_image().ok())
        .map(|i| (i.width(), i.height()))
        .max_by(|a, b| a.0.cmp(&b.0))
        .unwrap_or((512, 512));
    let target = icon.resize(width, height, image::imageops::Lanczos3);
    let new = icns::Image::from_data(
        icns::PixelFormat::RGBA,
        width,
        height,
        target.to_rgba8().to_vec(),
    )?;

    let mut icon_family = icns::IconFamily::new();
    icon_family.add_icon(&new)?;

    let mut file = vec![].writer();
    icon_family
        .write(&mut file)
        .map_err(|e| {
            println!("写入失败: {:?}", e);
            e
        })
        .unwrap();
    std::fs::write(icns, file.get_ref())?;
    Ok(())
}

#[test]
fn icon_generate() {
    let files = r#"
        128x128.png            
        128x128@2x.png         
        32x32.png              
        icon.icns              
        icon.ico               
        Square107x107Logo.png  
        Square142x142Logo.png  
        Square150x150Logo.png  
        Square284x284Logo.png  
        Square30x30Logo.png    
        Square310x310Logo.png  
        Square44x44Logo.png    
        Square71x71Logo.png    
        Square89x89Logo.png    
        StoreLogo.png
    "#;
    let icon = "./icons/icon.png";
    let exist = std::path::Path::new(icon).exists();
    assert!(exist);
    let icon = image::open(icon).unwrap();
    files.lines().map(|l| l.trim()).for_each(|f| {
        let f = format!("./icons/{}", f);
        let file = std::path::Path::new(&f);
        if !file.exists() {
            return;
        }
        if f.ends_with(".icns") {
            icns(&icon, &f).ok();
            return;
        }
        if let Ok(i) = image::open(&f) {
            let (width, height) = i.dimensions();
            let target = icon
                .resize(width, height, image::imageops::Lanczos3)
                .to_rgba8();
            target.save(f).ok();
        }
    });
    assert_eq!(1, 1)
}
