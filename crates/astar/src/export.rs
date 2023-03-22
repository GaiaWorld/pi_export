#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use pi_path_finding::{AStar, Entry, NormalTileMap, NodeIndex, make_neighbors};

/// A* 寻路网格数据 序号转换模式
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, Copy)]
pub enum AStarGridMode {
    NONE = 0,
    SCHOOL = 1
}

/// A* 寻路网格接口
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub struct AStarGrid {
    /// 底层网格数据
    tilemap: NormalTileMap<usize>,
    /// 测试用数据
    // list: Vec<usize>,
    /// 网格半宽度
    half_w: usize,
    /// 序号转换模式
    mode: AStarGridMode,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
impl AStarGrid {
	#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
	#[cfg(feature = "pi_js_export")]
    pub fn new(row: usize, column: usize, mode: u8) -> Self {
        use std::mem::transmute;

        AStarGrid {
            tilemap: NormalTileMap::new(row, column, 100),
            // list: Vec::default(),
            half_w: column / 2,
            mode: unsafe {transmute(mode)},
        }
    }
    ///
    /// 将 x, y 处点设置为障碍 - 底层 横纵 坐标
    #[cfg(feature = "pi_js_export")]
	pub fn disable_node(&mut self, x: usize, y: usize) {
        self.tilemap.nodes[x + y * self.tilemap.column] = true;
    }
    ///
    /// 将 x, y 处点设置为 非障碍 - 底层 横纵 坐标
    #[cfg(feature = "pi_js_export")]
	pub fn enable_node(&mut self, x: usize, y: usize) {
        self.tilemap.nodes[x + y * self.tilemap.column] = false;
    }
    /// 批量更新网格障碍标识 - 调用方的网格数据为 u8array
    #[cfg(feature = "pi_js_export")]
	pub fn update_by_u8array(&mut self, data: &mut [u8]) {
        let count = data.len();

        for i in 0..count {
            // 转换 调用方的 序号 为 底层序号
            let id = trans_id(i, self.half_w, self.mode);
            // 这里固定认为值为 0 标识为障碍块
            if data[i] == 0 {
                self.disable_index(id);
            }
            else {
                self.enable_index(id);
            }
        }
    }
    /// 批量更新网格障碍标识 - 调用方的网格数据为 u16array
    #[cfg(feature = "pi_js_export")]
	pub fn update_by_u16array(&mut self, data: &mut [u8]) {
		let data: &mut [u16] = bytemuck::cast_slice_mut(data);
        let count = data.len();

        for i in 0..count {
            // 转换 调用方的 序号 为 底层序号
            let id = trans_id(i, self.half_w, self.mode);
            // 这里固定认为值为 0 标识为障碍块
            if data[i] == 0 {
                self.disable_index(id);
            }
            else {
                self.enable_index(id);
            }
        }
    }
    /// 将 指定序号 id 处点设置为障碍 - 底层坐标
    #[cfg(feature = "pi_js_export")]
	pub fn disable_index(&mut self, index: usize) {
        self.tilemap.nodes[index] = true;
    }
    /// 将 指定序号 id 处点设置为障碍 - 底层坐标
    #[cfg(feature = "pi_js_export")]
	pub fn enable_index(&mut self, index: usize) {
        self.tilemap.nodes[index] = false;
    }
    /// 查找指定 id -> id 坐标的逻辑 - js层坐标
    #[cfg(feature = "pi_js_export")]
	pub fn find(&mut self, mut id0: usize, mut id1: usize, path: &mut [u8], max_len: usize) -> usize {
		let path: &mut [u32] = bytemuck::cast_slice_mut(path);
        // 转换 调用方的 序号 为 底层序号
        id0 = trans_id(id0, self.half_w, self.mode);
        id1 = trans_id(id1, self.half_w, self.mode);
        
        let map = &mut self.tilemap;

        let mut astar: AStar<usize, Entry<usize>> = AStar::with_capacity(map.column * map.row, (map.column + map.row) * 100);
        let start = NodeIndex(id0);
        let end = NodeIndex(id1);
        let _r = astar.find(start, end, max_len, &mut |cur, end, finder| {
            make_neighbors(map, cur, end, finder)
        });
        
        let mut path_length = 0usize;

        let mut last_x: usize = usize::MAX;
        let mut last_y: usize = usize::MAX;

        let mut diffx: usize = 0;
        let mut diffy: usize = 0;

        let mut last_id: usize = usize::MAX;
        let mut last_record: usize = usize::MAX;

        for node in astar.result_iter(end) {
            let x = node.0 % map.column;
            let y = node.0 / map.column;

            if path_length != 0 && x - last_x == diffx && y - last_y == diffy {

            } else {
                if last_id != last_record {
                    path[path_length] = trans_to_jsid(last_id, self.half_w, &self.mode) as u32;
                    path_length += 1;
                }
                // 转换 底层序号 为 调用方的 序号
                path[path_length] = trans_to_jsid(node.0, self.half_w, &self.mode) as u32;
                path_length += 1;

                diffx = x - last_x;
                diffy = y - last_y;

                last_record = node.0;
            }

            last_id = node.0;
            last_x = x;
            last_y = y;
        }

        if last_id != last_record {
            path[path_length] = trans_to_jsid(last_id, self.half_w, &self.mode) as u32;
            path_length += 1;
        }

        return path_length;
    }
}

///
/// 将js层ID转换为底层id
pub fn trans_id(jsid: usize, half_w: usize, mode: AStarGridMode) -> usize {
    match mode {
        AStarGridMode::NONE => {
            jsid
        },
        AStarGridMode::SCHOOL => {
            let ii = jsid / 4;
            let diff = jsid % 4;
        
            let h = ii / half_w;
            let w = ii % half_w;
        
            if diff == 0 {
                w * 2 + 2 * h * half_w * 2
            }
            else if diff == 1 {
                w * 2 + 1 + 2 * h * half_w * 2
            }
            else if diff == 2 {
                w * 2 + (2 * h + 1) * half_w * 2
            }
            else {
                w * 2 + 1 + (2 * h + 1) * half_w * 2
            }
        },
    }
}
/// 将底层id转换为js层ID
pub fn trans_to_jsid(id: usize, half_w: usize, mode: &AStarGridMode) -> usize {
    match *mode {
        AStarGridMode::NONE => {
            id
        },
        AStarGridMode::SCHOOL => {
            let column = half_w * 2;
            let tx = id % column;
            let ty = id / column;
            let base = (tx / 2 + ty / 2 * half_w) * 4;
            let diffx = tx % 2;
            let diffy = ty % 2;
        
            if diffx == 0 {
                if diffy == 0 {
                    base + 0
                }
                else {
                    base + 2
                }
            }
            else {
                if diffy == 0 {
                    base + 1
                }
                else {
                    base + 3
                }
            }
        },
    }
}