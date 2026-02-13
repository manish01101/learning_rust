// Image Processing Library Using Zero-Cost Abstractions
// Trait defining operations for images
trait Image {
    fn resize(&mut self, width: usize, height: usize);
    fn to_grayscale(&self) -> Vec<u8>;
}

// Struct representing a RGB image
struct RgbImage {
    width: usize,
    height: usize,
    data: Vec<u8>, // RGB data in a flat vector
}

impl RgbImage {
    fn new(width: usize, height: usize) -> Self {
        let data = vec![255; width * height * 3]; // Initialize with white pixels
        Self {
            width,
            height,
            data,
        }
    }
}

// Implement the Image trait for RgbImage
impl Image for RgbImage {
    fn resize(&mut self, width: usize, height: usize) {
        // Simple nearest-neighbor resizing (for demonstration purposes)
        let new_data = vec![0; width * height * 3]; // Create new data array
        self.width = width;
        self.height = height;
        self.data = new_data; // Replace old data with new data
    }

    fn to_grayscale(&self) -> Vec<u8> {
        // Convert RGB to grayscale
        self.data
            .chunks(3)
            .map(|pixel| {
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];
                (0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64) as u8
            })
            .collect()
    }
}

// Struct representing a Grayscale image
struct GrayImage {
    width: usize,
    height: usize,
    data: Vec<u8>, // Grayscale data in a flat vector
}

impl GrayImage {
    fn new(width: usize, height: usize) -> Self {
        let data = vec![0; width * height]; // Initialize with black pixels
        Self {
            width,
            height,
            data,
        }
    }
}

// Implement the Image trait for GrayImage
impl Image for GrayImage {
    fn resize(&mut self, width: usize, height: usize) {
        let new_data = vec![0; width * height]; // Create new data array
        self.width = width;
        self.height = height;
        self.data = new_data; // Replace old data with new data
    }

    fn to_grayscale(&self) -> Vec<u8> {
        self.data.clone() // Already in grayscale
    }
}

// Function to process images generically
fn process_images<I: Image>(images: &mut [I]) {
    for image in images {
        image.resize(100, 100); // Resize each image to 100x100
        let gray_data = image.to_grayscale(); // Convert to grayscale
        println!(
            "Processed image with {} pixels in grayscale.",
            gray_data.len()
        );
    }
}

fn main() {
    let mut rgb_image = RgbImage::new(200, 200);
    let mut gray_image = GrayImage::new(200, 200);

    // Create a mutable slice of images
    let mut images: Vec<&mut dyn Image> = vec![&mut rgb_image, &mut gray_image];

    // Process the images
    process_images(&mut images);
}
