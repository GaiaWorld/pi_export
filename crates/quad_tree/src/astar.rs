use bytemuck::NoUninit;
use js_proxy_gen_macro::pi_js_export;
use pi_path_finding::{
    base::{Aabb, Point},
    finder::AStarResult,
    finder::NodeIndex,
    normal::{make_neighbors, Entry},
    tile_map::{sort_by_dist, FlagTileMap, PathFilterIter, PathSmoothIter},
};

use std::mem::transmute;

// 瓦片标识类型
#[pi_js_export]
#[derive(Debug, Clone, PartialEq)]
pub enum TileFlagType {
    All = 0,
    Center = 1,
    Right = 2,
    Down = 4,
}

#[pi_js_export]
pub struct TileMap {
    inner: pi_path_finding::tile_map::TileMap,
    result: Vec<Point>,
}

impl TileMap {
    #[pi_js_export]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            inner: pi_path_finding::tile_map::TileMap::new(width as usize, height as usize, 100, 144),
            result: Default::default(),
        }
    }

    #[pi_js_export]
    pub fn set_node_flag(&mut self, index: u32, flag_type: TileFlagType, value: u8) -> u8 {
        if flag_type == TileFlagType::All {
            self.inner.set_node_flag(NodeIndex(index as usize), value)
        } else {
            self.inner
                .set_node_flag_type(NodeIndex(index as usize), unsafe { transmute(flag_type) }, value)
        }
    }

    #[pi_js_export]
    pub fn get_node_flag(&mut self, index: u32, flag_type: TileFlagType) -> u8 {
        if flag_type == TileFlagType::All {
            self.inner.get_node_flag(NodeIndex(index as usize))
        } else {
            self.inner
                .get_node_flag_type(NodeIndex(index as usize), unsafe { transmute(flag_type) })
        }
    }

    #[pi_js_export]
    pub fn set_range_flag(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, value: u8) {
        let aabb = Aabb::new(
            Point::new(x1 as isize, y1 as isize),
            Point::new(x2 as isize, y2 as isize),
        );
        self.inner.set_range_flag(&aabb, value)
    }
    // 获得指定点周围所有可用的点，周围是顺时针一圈一圈扩大，直到可用点数超过count, spacing为可用点的间隔
    // 参数d: Direction为默认扩展的朝向，0-3都可以设置

    #[pi_js_export]
    pub fn find_round(
        &mut self,
        index: u32,
        count: u32,
        spacing: u32,
        d: Direction,
        flag: u8,
    ) -> Vec<i32> {
        let dd: u8 = unsafe { transmute(d) };
        self.result.clear();
        let aabb = self.inner.find_round(
            NodeIndex(index as usize),
            count as usize,
            spacing as usize,
            unsafe { transmute(dd as u32) },
            flag,
            &mut self.result,
        );
        let mut result = Vec::new();
        if self.result.len() == 0 {
            result.push(aabb.min.x as i32);
            result.push(aabb.min.y as i32);
            result.push(aabb.max.x as i32);
            result.push(aabb.max.y as i32);
            return result;
        }
        let vec: &Vec<P> = unsafe { transmute(&self.result) };
        let rr: &[i64] = bytemuck::cast_slice(&vec.as_slice());
        let mut rr = rr.iter().map(|v| *v as i32).collect::<Vec<i32>>();
        result.append(&mut rr);
        return result;
    }
    // 寻找一个点周围可以放置的位置列表，并且按到target的距离进行排序（小-大）

    #[pi_js_export]
    pub fn find_round_and_sort_by_dist(
        &mut self,
        index: u32,
        count: u32,
        spacing: u32,
        d: Direction,
        flag: u8,
        target_x: isize,
        target_y: isize,
    ) -> Vec<i32> {
        let dd: u8 = unsafe { transmute(d) };
        self.result.clear();
        let aabb = self.inner.find_round(
            NodeIndex(index as usize),
            count as usize,
            spacing as usize,
            unsafe { transmute(dd as u32) },
            flag,
            &mut self.result,
        );
        let mut result = Vec::new();
        if self.result.len() == 0 {
            result.push(aabb.min.x as i32);
            result.push(aabb.min.y as i32);
            result.push(aabb.max.x as i32);
            result.push(aabb.max.y as i32);
            return result;
        }
        sort_by_dist(Point::new(target_x, target_y), &mut self.result);
        let vec: &Vec<P> = unsafe { transmute(&self.result) };
        let rr: &[i64] = bytemuck::cast_slice(&vec.as_slice());
        let mut rr = rr.iter().map(|v| *v as i32).collect::<Vec<i32>>();
        result.append(&mut rr);
        return result;
    }

    #[pi_js_export]
    pub fn test_line(
        &self,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        center_flag: u8,
        side_flag: u8,
    ) -> Option<Vec<i32>> {
        let map = FlagTileMap::new(&self.inner, center_flag, side_flag);
        pi_path_finding::tile_map::test_line(
            &map,
            Point::new(start_x as isize, start_y as isize),
            Point::new(end_x as isize, end_y as isize),
        )
        .map(|v| vec![v.x as i32, v.y as i32])
    }
}

// 四方向枚举
#[pi_js_export]
#[derive(Debug, Clone)]
pub enum Direction {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}
#[derive(Clone, Copy)]
struct P(i32, i32);

unsafe impl NoUninit for P {}

#[pi_js_export]
pub struct AStar {
    inner: pi_path_finding::finder::AStar<usize, Entry<usize>>,
}

impl AStar {
    #[pi_js_export]
    pub fn new(width: u32, height: u32, node_number: u32) -> Self {
        Self {
            inner: pi_path_finding::finder::AStar::with_capacity((width * height) as usize, node_number as usize),
        }
    }

    /*
    @brief: 求解路径
    @param tile_map: 地图
     */

    #[pi_js_export]
    pub fn find_path(
        &mut self,
        tile_map: &mut TileMap,
        max_number: u32,
        start: u32,
        end: u32,
        center_flag: u8,
        side_flag: u8,
    ) -> Option<u32> {
        let mut map = FlagTileMap::new(&tile_map.inner, center_flag, side_flag);

        let result = self.inner.find(
            NodeIndex(start as usize),
            NodeIndex(end as usize),
            max_number as usize,
            &mut map,
            make_neighbors,
        );
        match result {
            AStarResult::Found => return Some(end.clone() as u32),
            AStarResult::NotFound(index) => return Some(index.0 as u32),
            AStarResult::LimitNotFound(index) => return Some(index.0 as u32),
        };
    }

    /*
     * @brief: 输出路径
     * @param[in] node: 从路径的哪一个节点开始输出，一般情况下是终点
     * @param[in] tile_map: 地图
     * #return[in]: 路径数组
     */

    #[pi_js_export]
    pub fn result(
        &mut self,
        node: u32,
        tile_map: &mut TileMap,
        center_flag: u8,
        side_flag: u8,
    ) -> Vec<i32> {
        tile_map.result.clear();
        let map = FlagTileMap::new(&tile_map.inner, center_flag, side_flag);

        for r in PathSmoothIter::new(
            PathFilterIter::new(
                self.inner.result_iter(NodeIndex(node as usize)),
                tile_map.inner.width,
            ),
            &map,
        ) {
            tile_map.result.push(r);
        }
        let vec: &Vec<P> = unsafe { transmute(&tile_map.result) };
        let rr: &[isize] = bytemuck::cast_slice(&vec.as_slice());
        // path_result(rr)
        rr.iter().map(|v| *v as i32).collect()
    }
}
