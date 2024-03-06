#![cfg(not(doctest))]
#![doc = include_str!("../README.md")]

mod chess_assets;
pub use chess_assets::asset_manager::ChessAssets;
use image::imageops::overlay;
use std::io::Cursor;

/// Converts a FEN (Forsyth–Edwards Notation) string to a chess board image and saves it to a file.
///
/// # Arguments
///
/// * `fen` - The FEN string representing the chess position.
/// * `save_dir` - The directory where the generated image will be saved.
/// * `upscale_multiplier` - The multiplier for upscaling the image.
/// * `chess_assets` - The chess assets to use for generating the board image.
///
/// # Example
///
/// ```
/// use fenpix::fen_to_board_img;
///
/// fen_to_board_img("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", "chess_board.png", 2);
/// ```
pub fn fen_to_board_img(
    fen: &str,
    save_dir: &str,
    upscale_multiplier: u32,
    chess_assets: ChessAssets,
) {
    let board = fen.split_whitespace().next().unwrap();
    let mut img = chess_assets.board_image.clone();
    let square_size = (img.width() - 8) / 8; // Subtract border size from width before dividing by 8
    let piece_size = 16;
    let offset = (square_size - piece_size) / 2;
    let border_size = 4;
    let mut x = 0;
    let mut y = 0;
    for char in board.chars() {
        if char == '/' {
            y += 1;
            x = 0;
            continue;
        }
        if let Some(digit) = char.to_digit(10) {
            x += digit;
            continue;
        }
        if let Some(piece_image) = chess_assets.piece_images.get(&char) {
            overlay(
                &mut img,
                piece_image,
                (x * square_size + offset + border_size) as i64,
                (y * square_size + offset + border_size) as i64,
            );
        }
        x += 1;
    }

    let new_width = img.width() * upscale_multiplier;
    let new_height = img.height() * upscale_multiplier;
    let upscale_filter = image::imageops::FilterType::Nearest;

    let upscaled_img = image::imageops::resize(&img, new_width, new_height, upscale_filter);

    upscaled_img.save(save_dir).unwrap();
}

/// Converts a FEN (Forsyth–Edwards Notation) string to a chess board image and returns it as a buffer.
///
/// # Arguments
///
/// * `fen` - The FEN string representing the chess position.
/// * `upscale_multiplier` - The multiplier for upscaling the image.
/// * `chess_assets` - The chess assets to use for generating the board image.
///
/// # Returns
///
/// A vector of bytes representing the image buffer.
pub fn fen_to_board_buffer(
    fen: &str,
    upscale_multiplier: u32,
    chess_assets: ChessAssets,
) -> Vec<u8> {
    let board = fen.split_whitespace().next().unwrap();
    let mut img = chess_assets.board_image.clone();
    let square_size = (img.width() - 8) / 8; // Subtract border size from width before dividing by 8
    let piece_size = 16;
    let offset = (square_size - piece_size) / 2;
    let border_size = 4;
    let mut x = 0;
    let mut y = 0;
    for char in board.chars() {
        if char == '/' {
            y += 1;
            x = 0;
            continue;
        }
        if let Some(digit) = char.to_digit(10) {
            x += digit;
            continue;
        }
        if let Some(piece_image) = chess_assets.piece_images.get(&char) {
            overlay(
                &mut img,
                piece_image,
                (x * square_size + offset + border_size) as i64,
                (y * square_size + offset + border_size) as i64,
            );
        }
        x += 1;
    }

    let new_width = img.width() * upscale_multiplier;
    let new_height = img.height() * upscale_multiplier;
    let upscale_filter = image::imageops::FilterType::Nearest;

    let upscaled_img = image::imageops::resize(&img, new_width, new_height, upscale_filter);

    let mut buffer = Cursor::new(Vec::new());
    upscaled_img
        .write_to(&mut buffer, image::ImageOutputFormat::Png)
        .unwrap();

    buffer.into_inner()
}

#[cfg(test)]
mod tests {
    use crate::{chess_assets::asset_manager::ChessAssets, fen_to_board_img};

    #[test]
    fn fen_to_board_img_test() {
        fen_to_board_img(
            "5k1r/2q3p1/p3p2p/1B3p1Q/n4P2/6P1/bbP2N1P/1K1RR3",
            "chess_board.png",
            3,
            ChessAssets::default(),
        );
    }
}
