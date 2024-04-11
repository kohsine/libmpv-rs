use libmpv2::{events::*, *};

use std::{collections::HashMap, env, thread, time::Duration};

const VIDEO_URL: &str = "test-data/jellyfish.mp4";

fn main() -> Result<()> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from(VIDEO_URL));

    // Create an `Mpv` and set some properties.
    let mpv = Mpv::new()?;
    mpv.set_property("volume", 15)?;
    mpv.set_property("vo", "null")?;

    let mut ev_ctx = EventContext::new(mpv.ctx);
    ev_ctx.disable_deprecated_events()?;
    ev_ctx.observe_property("volume", Format::Int64, 0)?;
    ev_ctx.observe_property("demuxer-cache-state", Format::Node, 0)?;

    crossbeam::scope(|scope| {
        scope.spawn(|_| {
            mpv.playlist_load_files(&[(&path, FileState::AppendPlay, None)])
                .unwrap();

            thread::sleep(Duration::from_secs(3));

            mpv.set_property("volume", 25).unwrap();

            thread::sleep(Duration::from_secs(5));

            // Trigger `Event::EndFile`.
            mpv.playlist_next_force().unwrap();
        });
        scope.spawn(move |_| loop {
            let ev = ev_ctx.wait_event(600.).unwrap_or(Err(Error::Null));

            match ev {
                Ok(Event::EndFile(r)) => {
                    println!("Exiting! Reason: {:?}", r);
                    break;
                }

                Ok(Event::PropertyChange {
                    name: "demuxer-cache-state",
                    change: PropertyData::Node(mpv_node),
                    ..
                }) => {
                    let ranges = seekable_ranges(mpv_node).unwrap();
                    println!("Seekable ranges updated: {:?}", ranges);
                }
                Ok(e) => println!("Event triggered: {:?}", e),
                Err(e) => println!("Event errored: {:?}", e),
            }
        });
    })
    .unwrap();
    Ok(())
}

fn seekable_ranges(demuxer_cache_state: &MpvNode) -> Option<Vec<(f64, f64)>> {
    let mut res = Vec::new();
    let props: HashMap<&str, MpvNode> = demuxer_cache_state.to_map()?.collect();
    let ranges = props.get("seekable-ranges")?.to_array()?;

    for node in ranges {
        let range: HashMap<&str, MpvNode> = node.to_map()?.collect();
        let start = range.get("start")?.to_f64()?;
        let end = range.get("end")?.to_f64()?;
        res.push((start, end));
    }

    Some(res)
}
