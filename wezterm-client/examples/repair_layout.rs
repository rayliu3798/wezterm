//! Restore pane geometry through the existing mux resize API; never sends input.
use anyhow::{Context, Result};
use codec::Resize;
use config::UnixDomain;
use mux::connui::ConnectionUI;
use std::path::PathBuf;
use wezterm_client::client::Client;
use wezterm_term::TerminalSize;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let socket = PathBuf::from(args.next().context("missing explicit GUI socket")?);
    let mut ui = ConnectionUI::new_headless();
    let client = Client::new_unix_domain(
        None,
        &UnixDomain {
            socket_path: Some(socket),
            no_serve_automatically: true,
            ..Default::default()
        },
        false,
        &mut ui,
        true,
    )?;
    for request in args {
        let values = request
            .split(',')
            .map(str::parse::<usize>)
            .collect::<std::result::Result<Vec<_>, _>>()?;
        anyhow::ensure!(values.len() == 7, "expected tab,pane,rows,cols,pixelsX,pixelsY,dpi");
        smol::block_on(client.resize(Resize {
            containing_tab_id: values[0],
            pane_id: values[1],
            size: TerminalSize {
                rows: values[2],
                cols: values[3],
                pixel_width: values[4],
                pixel_height: values[5],
                dpi: values[6] as _,
            },
        }))?;
        println!("Resized pane {} to {} rows, {} columns", values[1], values[2], values[3]);
    }
    Ok(())
}
