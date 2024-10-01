use std::path::PathBuf;
use thiserror::Error;

/**
 * Contains `game.rs` related errors.
 */
#[derive(Debug, Error)]
pub enum GameError {
    /// When a value is **not** in the range `[0; Game.side_size]`.
    #[error("Illegal value not in [0; side_size].")]
    IllegalValue,
    /// When an existing cell prevents a value from being set.
    #[error("Invalid value for this cell.")]
    InvalidValue,
    /// When an asked position is out of the grid.
    #[error("This cell position is invalid.")]
    IllegalPosition,
    /// When trying to set a value in a cell that already contains a non modifiable (initial) value.
    #[error("This cell already contain a value.")]
    NonEmptyCell,
    /// Occurs when there is an error during the save file creation.
    #[error("Unable to create the save file.")]
    CreateSaveFileError,
    /// Occurs when trying to save a game that does not have an attached file.
    #[error("Cannot save the game because it does not have a save file attached.")]
    NoSaveFile,
    /// Occurs when there is an error whilst writing the game save in a file.
    #[error("Unable to save to file.")]
    WriteSaveError,
    /// Occurs when unable to read a save file content.
    #[error("Unable to read the save file content.")]
    OpenFileError,
    /// Occurs when there is an error whilst parsing a save file.
    #[error("Unable to parse the save file correctly.")]
    ParseSaveFileError,
    /// Occurs when the save file was loaded but contains erroneous values.
    #[error("The save file contains erroneous data.")]
    IncorrectSaveFile,
    /// Occurs when unable to open an existing save file.
    #[error("Unable to open the save file again.")]
    OpenSaveFileError,
}

/**
 * Contains `solver.rs` related errors.
 */
#[derive(Debug, Error)]
pub enum SolverError {
    /// When a solver does not succeed in solving a game.
    #[error("Failed to solve the grid.")]
    FailedToSolve,
}

/**
 * Contains errors related to the `Ui` trait of `ui.rs`.
 *
 * TODO: Implement the From::<GuiError> trait (or the other way around) so that the implementations
 * of Gui and Cli can return their own errors.
 */
#[derive(Debug, Error)]
pub enum UiError {
    /// Occurs when there is a an error whilst loading the configuration file.
    #[error("Failed to load the configuration file at [{0}]: {1}.")]
    LoadConfigError(PathBuf, String),
    /// Occurs when there is a syntax error in the configuration file.
    #[error("Syntax error in configuration file at [{0}]: {1}.")]
    ConfigSyntaxError(PathBuf, String),
    /// Occurs when there is an error whilst loading the font file.
    #[error("Unable to load font file.")]
    LoadFontError,
    #[error("Failed to load the sprite.")]
    LoadSpriteError,
    /// Occurs when there is an error during the save file creation.
    #[error("Unable to create the save file.")]
    CreateSaveFileError,
    /// Occurs when the loaded textures are missing a texture.
    #[error("Missing loaded texture.")]
    MissingLoadedTexture,
    /// Occurs when a SDL2 error occurs.
    #[error("Generic SDL2 Error")]
    SDL2Error,
    /// Occurs when there is an error writing the updated configuration file.
    #[error("An error occured when trying to write the updated configuration file.")]
    WriteConfigError,
}

impl From<GameError> for UiError {
    fn from(game_error: GameError) -> Self {
        match game_error {
            GameError::CreateSaveFileError => Self::CreateSaveFileError,
            _ => todo!(),
        }
    }
}
