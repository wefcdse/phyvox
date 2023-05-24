use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use bevy::prelude::*;

use crate::chunk::{
    chunk::{simple_generator::SimpleGenerator, Chunk, ChunkGenerator, Seed, CHUNK_SIZE},
    Pos,
};

use super::Player;

#[derive(Debug, Clone, PartialEq, Resource)]
pub struct GeneratorInfo {
    pub range_xz: u64,
    pub range_yp: u64,
    pub range_yn: u64,
    pub range_r: u64,
    pub seed: Seed,
    pub load_how_many_chunks_per_frame: usize,
    pub unload_how_many_chunks_per_frame: usize,
}

#[derive(Debug, Clone, PartialEq, Default, Resource)]
pub struct AllChunks {
    chunks: HashMap<Pos, Entity>,
}

pub fn startup(mut commands: Commands) {
    commands.insert_resource(GeneratorInfo {
        range_xz: 16,
        range_yp: 3,
        range_yn: 2,
        range_r: 16,
        seed: Seed { seed: 3 },
        load_how_many_chunks_per_frame: 50,
        unload_how_many_chunks_per_frame: 50,
    });
    commands.insert_resource(AllChunks::default());
}

pub fn new_chunks(
    playes: Query<&Transform, With<Camera>>,
    mut all_chunks: ResMut<AllChunks>,
    generatier_info: Res<GeneratorInfo>,
    mut commands: Commands,
) {
    playes.for_each(|transform| {
        let center_pos = Pos::from_xyz(
            transform.translation.x as i64,
            transform.translation.y as i64,
            transform.translation.z as i64,
        );

        let center_pos = Pos::from_xyz(
            center_pos.x() - (center_pos.x().rem_euclid(CHUNK_SIZE as i64)),
            center_pos.y() - (center_pos.y().rem_euclid(CHUNK_SIZE as i64)),
            center_pos.z() - (center_pos.z().rem_euclid(CHUNK_SIZE as i64)),
        );

        let r1 = Pos::from_xyz(
            generatier_info.range_xz as i64,
            generatier_info.range_yp as i64,
            generatier_info.range_xz as i64,
        );
        let r2 = -Pos::from_xyz(
            generatier_info.range_xz as i64,
            generatier_info.range_yn as i64,
            generatier_info.range_xz as i64,
        );
        let mut uploaded = 0_usize;
        for i in Pos::iter_range(r1, r2).filter(|p| {
            p.x() * p.x() + p.y() * p.y() + p.z() * p.z()
                < (generatier_info.range_r * generatier_info.range_r) as i64
        }) {
            //dbg!(i);
            let base = center_pos + i * CHUNK_SIZE as i64;
            if !all_chunks.chunks.contains_key(&base) {
                // dbg!(base == base);
                // dbg!(base);
                let e = commands
                    .spawn(SimpleGenerator.generate_chunk(base, generatier_info.seed))
                    .insert(TransformBundle::from_transform(
                        Transform::from_translation(Vec3 {
                            x: base.x() as f32,
                            y: base.y() as f32,
                            z: base.z() as f32,
                        }),
                    ))
                    .id();

                all_chunks.chunks.insert(base, e);
                // dbg!(&all_chunks.chunks);
                uploaded += 1;
                if uploaded >= generatier_info.load_how_many_chunks_per_frame {
                    break;
                }
            }
        }
    });
}

pub fn delete_chunks(
    playes: Query<&Transform, With<Camera>>,
    mut all_chunks: ResMut<AllChunks>,
    generatier_info: Res<GeneratorInfo>,
    mut commands: Commands,
) {
    playes.for_each(|transform| {
        let center_pos = Pos::from_xyz(
            transform.translation.x as i64,
            transform.translation.y as i64,
            transform.translation.z as i64,
        );

        let center_pos = Pos::from_xyz(
            center_pos.x() - (center_pos.x().rem_euclid(CHUNK_SIZE as i64)),
            center_pos.y() - (center_pos.y().rem_euclid(CHUNK_SIZE as i64)),
            center_pos.z() - (center_pos.z().rem_euclid(CHUNK_SIZE as i64)),
        );

        let r1 = Pos::from_xyz(
            generatier_info.range_xz as i64,
            generatier_info.range_yp as i64,
            generatier_info.range_xz as i64,
        );
        let r2 = -Pos::from_xyz(
            generatier_info.range_xz as i64,
            generatier_info.range_yn as i64,
            generatier_info.range_xz as i64,
        );
        let v = Pos::iter_range(r1, r2)
            .filter(|p| {
                p.x() * p.x() + p.y() * p.y() + p.z() * p.z()
                    < (generatier_info.range_r * generatier_info.range_r) as i64
            })
            .map(|i| {
                {
                    //dbg!(i);
                    let base = center_pos + i * CHUNK_SIZE as i64;
                    base
                }
            })
            .collect::<HashSet<_>>();

        let to_delete = all_chunks
            .chunks
            .iter()
            .filter(|(k, _)| !v.contains(k))
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<_>>();
        for (i, (k, v)) in to_delete.into_iter().enumerate() {
            if i >= generatier_info.unload_how_many_chunks_per_frame {
                return;
            }
            if let Some(mut e) = commands.get_entity(v) {
                e.despawn();
            }
            all_chunks.chunks.remove(&k);
        }
    });
}

#[test]
fn a() {
    dbg!(3 % 10);
    dbg!((-3) % 10);
    dbg!(3 / 10);
    dbg!((-3) / 10);
    dbg!(3_i32.rem_euclid(10));
    dbg!((-3_i32).rem_euclid(10));
}
