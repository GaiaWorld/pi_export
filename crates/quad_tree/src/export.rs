
// extern crate nalgebra;
// extern crate pi_spatialtree;
// extern crate ncollide2d;
// extern crate num_traits;


// use core::f32;
// use nalgebra::{Matrix4, Point2, RealField, Scalar, Vector2, Vector4};
// use ncollide2d::bounding_volume::AABB;
// use pi_spatialtree::quad_helper::QuadTree;

// // // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// // // allocator.
// // #[cfg(feature = "wee_alloc")]
// // #[global_allocator]
// // static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[cfg(target_arch = "wasm32")]
// use wasm_bindgen::prelude::*;

// struct MeshInfo {
//     pub id: usize,
//     pub minx: f32,
//     pub maxx: f32,
//     pub miny: f32,
//     pub maxy: f32,
//     pub minv: Vector4<f32>,
//     pub maxv: Vector4<f32>,
//     pub dirty: bool,
// }

// impl MeshInfo {
//     fn new(p: Matrix4<f32>, min: &[f32], max: &[f32], id: usize) -> MeshInfo {
//         let minv = Vector4::new(min[0], min[1], min[2], 1.0f32);
//         let maxv = Vector4::new(max[0], max[1], max[2], 1.0f32);

//         let pmin = p * minv;
//         let pmax = p * maxv;
//         let minx = pmin.x.min(pmax.x);
//         let miny = pmin.y.min(pmax.y);
//         let maxx = pmin.x.max(pmax.x);
//         let maxy = pmin.y.max(pmax.y);

//         MeshInfo {
//             id,
//             minx,
//             miny,
//             maxx,
//             maxy,
//             minv,
//             maxv,
//             dirty: true
//         }
//     }
//     pub fn reset(&mut self, min: &[f32], max: &[f32]) {
//         self.minv.x = min[0];
//         self.minv.y = min[1];
//         self.minv.z = min[2];
//         self.maxv.x = max[0];
//         self.maxv.y = max[1];
//         self.maxv.z = max[2];

//         self.dirty = true;
//     }
//     pub fn compute(&mut self, p: Matrix4<f32>) {
//         let pmin = p * self.minv;
//         let pmax = p * self.maxv;

//         self.minx = pmin.x.min(pmax.x);
//         self.miny = pmin.y.min(pmax.y);
//         self.maxx = pmin.x.max(pmax.x);
//         self.maxy = pmin.y.max(pmax.y);

//         // println!(">>>>>>>>>>>>>>>>>>>");
//         // println!("self.minv: {:?}", self.minv);
//         // println!("self.maxv: {:?}", self.maxv);
//         // println!("pmin: {:?}", pmin);
//         // println!("pmax: {:?}", pmax);

//         self.dirty = false;
//     }
//     pub fn unuse(&mut self) {
//         self.dirty = false;
//     }
// }

// #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
// #[cfg(feature = "pi_js_export")]
// pub struct RQuadTree {
//     id_max: usize,
//     id_list: Vec<usize>,
//     vp: Matrix4<f32>,
//     list: Vec<MeshInfo>,
//     tree: QuadTree<f32, usize>,
//     args: AbQueryArgs<f32>,
//     dirty: bool,
// }

// #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
// impl RQuadTree {
//     /// 创建四叉树接口
//     /// * @param p: viewMatrix - 相机的视图矩阵 - [f32; 16]
//     /// * @param width: 相机宽度(视野矩形宽度的一半)
//     /// * @param height: 相机高度(视野矩形高度的一半)
//     /// * @param maxw: 视野最大宽度(视野矩形宽度的一半)
//     /// * @param maxh: 视野最大高度(视野矩形高度的一半)
//     /// * @param deep: 四叉树深度
// 	#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
// 	#[cfg(feature = "pi_js_export")]
//     pub fn new(p: &[u8], width: f32, height: f32, maxw: f32, maxh: f32, deep: f32) -> RQuadTree {
// 		let p = bytemuck::cast_slice(p);
//         let mut vp = Matrix4::<f32>::default();

//         let max = Vector2::new(100f32, 100f32);
//         let min = max / 100f32;

//         for i in 0..16 {
//             vp[i] = p[i];
//         }

//         let tree = QuadTree::new(
//             AABB::new(
//                 Point2::new(-maxw, -maxh),
//                 Point2::new(maxw, maxh),
//             ),
//             max,
//             min,
//             0,
//             0,
//             f32::floor(deep) as usize,
//         );

//         let args = AbQueryArgs::<f32>::new(AABB::new(
//             Point2::new(-width, -height),
//             Point2::new(width, height),
//         ));

//         RQuadTree {
//             vp,
//             list: Vec::default(),
//             id_max: 0,
//             id_list: Vec::default(),
//             tree,
//             args,
//             dirty: false
//         }
//     }

//     /// 重置相机信息
//     /// * @param p: 相机视图矩阵 - [f32; 16]
//     /// * @param width: 相机视野宽度(视野矩形宽度的一半)
//     /// * @param height: 相机视野高度(视野矩形高度的一半)
// 	#[cfg(feature = "pi_js_export")]
//     pub fn reset(&mut self, p: &[u8], width: f32, height: f32) {
// 		let p = bytemuck::cast_slice(p);
//         for i in 0..16 {
//             self.vp[i] = p[i];
//         }

//         self.args.aabb.mins.x = -width;
//         self.args.aabb.mins.y = -height;
//         self.args.aabb.maxs.x = width;
//         self.args.aabb.maxs.y = height;

//         self.dirty = true;
//     }

//     /// 添加一个物件的包围盒(全局)
//     /// * @param min: min world point - [f32; 3]
//     /// * @param max: max world point - [f32; 3]
//     #[cfg(feature = "pi_js_export")]
// 	pub fn add(&mut self, min: &[u8], max: &[u8]) -> u32 {
// 		let min = bytemuck::cast_slice(min);
// 		let max = bytemuck::cast_slice(max);
//         let id: usize;
//         let info: &mut MeshInfo;

//         if self.id_list.len() > 0 {
//             id = self.id_list.pop().unwrap();

//             info = self.list.get_mut(id).unwrap();
//             info.reset(min, max);
//         }
//         else {
//             id = self.id_max;
//             self.id_max += 1;

//             let temp = MeshInfo::new(self.vp, min, max, id);
//             self.list.push(temp);

//             info = self.list.get_mut(id).unwrap();
//         }

//         info.compute(self.vp);
//         // println!("add {:?}", info.id);
//         self.tree.add(info.id + 1, AABB::new(Point2::new(info.minx, info.miny), Point2::new(info.maxx, info.maxy)), info.id);

//         return id as u32;
//     }

//     /// 更新指定物件的包围盒(全局)
//     /// * @param uuid: 目标id
//     /// * @param min: min world point - [f32; 3]
//     /// * @param max: max world point - [f32; 3]
//     #[cfg(feature = "pi_js_export")]
// 	pub fn update(&mut self, uuid: u32, min: &[u8], max: &[u8]) {
// 		let min = bytemuck::cast_slice(min);
// 		let max = bytemuck::cast_slice(max);
//         let info = self.list.get_mut(uuid as usize).unwrap();

//         info.reset(min, max);

//         if !self.dirty {
//             info.compute(self.vp);
//             self.tree.update(info.id + 1, AABB::new(Point2::new(info.minx, info.miny), Point2::new(info.maxx, info.maxy)));
//         }
//     }

//     /// 移除指定物件的包围盒(全局)
//     /// * @param uuid: 目标id
//     #[cfg(feature = "pi_js_export")]
// 	pub fn remove(&mut self, uuid: u32) {
//         let info = self.list.get_mut(uuid as usize).unwrap();

//         self.id_list.push(info.id as usize);
//         self.tree.remove(info.id + 1);

//         info.unuse();
//     }

//     /// 查询可见物件
//     /// * @param result - 数组长度应大于等于已设置物件包围盒数目
//     /// * @param max_len - 结果数组的长度
//     /// * @return 返回可见数目
//     #[cfg(feature = "pi_js_export")]
// 	pub fn query(&mut self, result: &mut [u8], max_len: u32) -> u32 {
// 		let result: &mut [u32] = bytemuck::cast_slice_mut(result);//[u32]
//         let tree = &mut self.tree;
//         let args = &mut self.args;

//         if self.dirty {
//             for info in self.list.iter_mut() {
//                 info.compute(self.vp);
//                 tree.update(info.id + 1, AABB::new(Point2::new(info.minx, info.miny), Point2::new(info.maxx, info.maxy)));
//             }
            
//             tree.collect();
//         }
//         else {
//             let mut flag = false;
//             for info in self.list.iter_mut() {
//                 if info.dirty {
//                     info.compute(self.vp);
//                     tree.update(info.id + 1, AABB::new(Point2::new(info.minx, info.miny), Point2::new(info.maxx, info.maxy)));
//                     flag = true;
//                 }
//             }

//             if flag {
//                 tree.collect();
//             }
//         }

//         tree.query(&args.aabb.clone(), intersects, args, ab_query_func);

//         self.dirty = false;

//         let mut index = 0usize;
//         for id in &args.result {
//             result[index] = *id as u32;
//             index +=1;
//             if max_len as usize == index {
//                 break;
//             }
//         }

//         return index as u32;
//     }
// }

// #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
// #[cfg(feature = "pi_js_export")]
// pub struct SQuadTree {
//     tree: QuadTree<f32, usize>,
//     args: AbQueryArgs<f32>,
// }

// #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
// #[cfg(feature = "pi_js_export")]
// impl SQuadTree {
//     /// 创建 静态四叉查询树
//     /// data 数据Buffer
//     /// data_block_count buffer 包含多少个数据块
//     /// data_block_size 一个数据块的 f32 数目
//     /// _min_max_data_size 一个数据块的 中 min 或 max 数据的 f32 数目 - 只能是 2
//     /// box_min_offset 一个数据块的 中 min 数据的 f32 偏移
//     /// box_max_offset 一个数据块的 中 max 数据的 f32 偏移
// 	#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
// 	#[cfg(feature = "pi_js_export")]
//     pub fn new(data: &[u8], data_block_count: u32, data_block_size: u32, _min_max_data_size: u32, box_min_offset: u32, box_max_offset: u32) -> Self {
//         let data: &[f32] = bytemuck::cast_slice(data);
// 		let mut min_x = f32::MAX;
//         let mut max_x = f32::MIN;
//         let mut min_y = f32::MAX;
//         let mut max_y = f32::MIN;

//         let mut minmax_list: Vec<(f32, f32, f32, f32)> = Vec::default();

//         for i in 0..data_block_count {
//             let index = i * data_block_size;

//             let mut f32index = (index + box_min_offset) as usize;
//             let minx = *data.get(f32index).unwrap();
//             let miny = *data.get(f32index + 1).unwrap();
            
//             f32index = (index + box_max_offset) as usize;
//             let maxx = *data.get(f32index).unwrap();
//             let maxy = *data.get(f32index + 1).unwrap();

//             min_x = min_x.min(minx);
//             min_y = min_y.min(miny);
//             max_x = max_x.max(maxx);
//             max_y = max_y.max(maxy);

//             minmax_list.push((minx, miny, maxx, maxy));
//         }

//         let max_loose = Vector2::new(max_x - min_x, max_y - min_y);
//         let min_loose = max_loose / 20f32;

//         let mut tree = QuadTree::new(
//             AABB::new(
//                 Point2::new(min_x, min_y),
//                 Point2::new(max_x, max_y),
//             ),
//             max_loose,
//             min_loose,
//             4usize,
//             40usize,
//             (data_block_count / 16) as usize,
//         );

//         let args = AbQueryArgs::<f32>::new(AABB::new(
//             Point2::new(-1.0, -1.0),
//             Point2::new(1.0, 1.0),
//         ));

//         for i in 0..data_block_count {
//             let minmax = minmax_list.get(i as usize).unwrap();
//             tree.add((i + 1) as usize, AABB::new(Point2::new(minmax.0, minmax.1), Point2::new(minmax.2, minmax.3)), i as usize);
//         }

//         SQuadTree {
//             tree,
//             args
//         }
//     }

//     /// 查询AABB范围 包含的 AABB id
//     /// minx - miny - maxx - maxy : 查询AABB
//     /// result : 包含的 AABB id 结果数组
//     /// max_len : 包含的 AABB id 结果数组最大长度
//     /// return 查询结果的数目
//     #[cfg(feature = "pi_js_export")]
// 	pub fn query(
//         &mut self,
//         minx: f32,
//         miny: f32,
//         maxx: f32,
//         maxy: f32,
//         result: &mut [u8],
//         max_len: u32
//     ) -> u32 {
// 		let result: &mut [u32] = bytemuck::cast_slice_mut(result);
//         self.args.aabb = AABB::new(Point2::new(minx, miny), Point2::new(maxx, maxy));
//         self.args.result.clear();
//         self.tree.query(&self.args.aabb.clone(), intersects, &mut self.args, ab_query_func);
        
//         let mut index = 0usize;
//         for id in &self.args.result {
//             result[index] = *id as u32 - 1;
//             index +=1;
//             if max_len as usize == index {
//                 break;
//             }
//         }

//         return index as u32;
//     }
// }

// struct AbQueryArgs<S: Scalar + RealField + Copy> {
//     pub aabb: AABB<S>,
//     pub result: Vec<usize>,
// }
// impl<S: Scalar + RealField + Copy> AbQueryArgs<S> {
//     pub fn new(aabb: AABB<S>) -> AbQueryArgs<S> {
//         AbQueryArgs {
//             aabb: aabb,
//             result: Vec::new(),
//         }
//     }
// }

// /// ab节点的查询函数, 这里只是一个简单范本，使用了oct节点的查询函数intersects
// /// 应用方为了功能和性能，应该实现自己需要的ab节点的查询函数， 比如点查询， 球查询-包含或相交， 视锥体查询...
// fn ab_query_func<S: Scalar + RealField + Copy, T: Clone>(
//     arg: &mut AbQueryArgs<S>,
//     id: usize,
//     aabb: &AABB<S>,
//     _bind: &T,
// ) {
//     // println!("aabb_query_func");
//     if intersects(&arg.aabb, aabb) {
//         arg.result.push(id);
//     }
// }
// /// quad节点查询函数的范本，aabb是否相交，参数a是查询参数，参数b是quad节点的aabb， 所以最常用的判断是左闭右开
// /// 应用方为了功能和性能，应该实现自己需要的quad节点的查询函数， 比如点查询， 球查询， 视锥体查询...
// #[inline]
// fn intersects<S: Scalar + RealField + Copy>(a: &AABB<S>, b: &AABB<S>) -> bool {
//     // println!("a.mins.x <= b.maxs.x >>> {:?} & {:?}", a.mins.x, b.maxs.x);
//     // println!("a.maxs.x > b.mins.x >>> {:?} & {:?}", a.maxs.x, b.mins.x);
//     // println!("a.mins.y <= b.maxs.y >>> {:?} & {:?}", a.mins.y, b.maxs.y);
//     // println!("a.maxs.y > b.mins.y >>> {:?} & {:?}", a.maxs.y, b.mins.y);

//     a.mins.x <= b.maxs.x
//         && a.maxs.x > b.mins.x
//         && a.mins.y <= b.maxs.y
//         && a.maxs.y > b.mins.y
// }