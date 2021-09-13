use log::info;
use mogwai::prelude::*;

use crate::game::Grid;

pub fn grid_view(grid: &Grid) -> ViewBuilder<HtmlElement> {
    builder! {
       <div class="rmg__fold">
            {
                format!("{:?}", grid.data)
            }
        </div>
    }
}
