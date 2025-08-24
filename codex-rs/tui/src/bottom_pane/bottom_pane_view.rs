use crate::user_approval_widget::ApprovalRequest;
use crossterm::event::KeyEvent;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

use super::BottomPane;
use super::CancellationEvent;

/// Trait implemented by every view that can be shown in the bottom pane.
pub(crate) trait BottomPaneView {
    /// Handle a key event while the view is active. A redraw is always
    /// scheduled after this call.
    fn handle_key_event(&mut self, _pane: &mut BottomPane, _key_event: KeyEvent) {}

    /// Return `true` if the view has finished and should be removed.
    fn is_complete(&self) -> bool {
        false
    }

    /// Handle Ctrl-C while this view is active.
    fn on_ctrl_c(&mut self, _pane: &mut BottomPane) -> CancellationEvent {
        CancellationEvent::Ignored
    }

    /// Return the desired height of the view.
    fn desired_height(&self, width: u16) -> u16;

    /// Render the view: this will be displayed in place of the composer.
    fn render(&self, area: Rect, buf: &mut Buffer);

    /// Update the status indicator animated header. Default no-op.
    fn update_status_header(&mut self, _header: String) {
        // no-op
    }

    /// Called when task completes to check if the view should be hidden.
    fn should_hide_when_task_is_done(&mut self) -> bool {
        false
    }

    /// Try to handle approval request; return the original value if not
    /// consumed.
    fn try_consume_approval_request(
        &mut self,
        request: ApprovalRequest,
    ) -> Option<ApprovalRequest> {
        Some(request)
    }

    /// Optional hook for views that expose a live status line. Views that do not
    /// support this can ignore the call.
    fn update_status_text(&mut self, _text: String) {}
}
