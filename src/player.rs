use rltk::{VirtualKeyCode, Rltk};
use specs::prelude::*;
use std::cmp::{max, min};

use super::{Position, Player, TileType, State, Map};
use crate::Viewshed;

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));

            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H
            => try_move_player(-1, 0, &mut gs.ecs),

            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L
            => try_move_player(1, 0, &mut gs.ecs),

            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K
            => try_move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J
            => try_move_player(0, 1, &mut gs.ecs),

            VirtualKeyCode::Numpad7 | VirtualKeyCode::Y
            => try_move_player(-1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad9 | VirtualKeyCode::U
            => try_move_player(1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad1 | VirtualKeyCode::B
            => try_move_player(-1, 1, &mut gs.ecs),

            VirtualKeyCode::Numpad3 | VirtualKeyCode::N
            => try_move_player(1, 1, &mut gs.ecs),

            _ => {}
        }
    }
}
