// Copyright (C) 2016  ParadoxSpiral
//
// This file is part of mpv-rs.
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA

#[cfg(feature = "events_complex")]
/// TODO: Doc
pub mod events_complex;
#[cfg(feature = "events_simple")]
/// TODO: Doc
pub mod events_simple;

/// An `Event`'s ID.
pub use raw::mpv_event_id as EventId;
pub mod mpv_event_id {
    #![allow(missing_docs)]
    pub use raw::mpv_event_id_MPV_EVENT_AUDIO_RECONFIG as AudioReconfig;
    pub use raw::mpv_event_id_MPV_EVENT_CLIENT_MESSAGE as ClientMessage;
    pub use raw::mpv_event_id_MPV_EVENT_COMMAND_REPLY as CommandReply;
    pub use raw::mpv_event_id_MPV_EVENT_END_FILE as EndFile;
    pub use raw::mpv_event_id_MPV_EVENT_FILE_LOADED as FileLoaded;
    pub use raw::mpv_event_id_MPV_EVENT_GET_PROPERTY_REPLY as GetPropertyReply;
    pub use raw::mpv_event_id_MPV_EVENT_HOOK as Hook;
    pub use raw::mpv_event_id_MPV_EVENT_IDLE as Idle;
    pub use raw::mpv_event_id_MPV_EVENT_LOG_MESSAGE as LogMessage;
    pub use raw::mpv_event_id_MPV_EVENT_NONE as None;
    pub use raw::mpv_event_id_MPV_EVENT_PLAYBACK_RESTART as PlaybackRestart;
    pub use raw::mpv_event_id_MPV_EVENT_PROPERTY_CHANGE as PropertyChange;
    pub use raw::mpv_event_id_MPV_EVENT_SEEK as Seek;
    pub use raw::mpv_event_id_MPV_EVENT_SET_PROPERTY_REPLY as SetPropertyReply;
    pub use raw::mpv_event_id_MPV_EVENT_SHUTDOWN as Shutdown;
    pub use raw::mpv_event_id_MPV_EVENT_START_FILE as StartFile;
    pub use raw::mpv_event_id_MPV_EVENT_TICK as Tick;
    pub use raw::mpv_event_id_MPV_EVENT_QUEUE_OVERFLOW as QueueOverflow;
    pub use raw::mpv_event_id_MPV_EVENT_VIDEO_RECONFIG as VideoReconfig;
}
