#!/bin/bash -x
#SBATCH --account=training2442
#SBATCH --threads-per-core=1
#SBATCH --nodes=2
#SBATCH --ntasks-per-node=4
#SBATCH --cpus-per-task=32
#SBATCH --time=04:00:00
#SBATCH --partition=batch

export RAYON_NUM_THREADS=${SLURM_CPUS_PER_TASK}
srun --exclusive target/release/XY-Model --run_id 1 --one 16 32 48 64 --two 32 64 96 128 160 192 224 256 288 320 352 384
