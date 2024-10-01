extern crate sdl2;
use anyhow::Context;
use clap::Parser;
use log::info;
use sdl2::image::InitFlag;

use std::path::PathBuf;

use rs_sudoku::gui;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    config_path: PathBuf,
}

pub fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    info!("Using configuration at [{}]", args.config_path.display());

    // Init SDL Context
    let sdl_context = sdl2::init().map_err(anyhow::Error::msg)?;
    // Init TTF Context
    let ttf_context = sdl2::ttf::init().context("SDL2 TTF context init")?;

    // Init SDL Image Context
    sdl2::image::init(InitFlag::PNG).map_err(anyhow::Error::msg)?;

    // Create a GUI instance.
    let mut gui =
        gui::Gui::new(&sdl_context, &ttf_context, &args.config_path).context("Creating GUI")?;

    // Try initiating the GUI
    gui.init().context("Initialising GUI")?;

    gui.run().context("Running the GUI")?;

    Ok(())
}
