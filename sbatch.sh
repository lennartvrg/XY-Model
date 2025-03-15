#!/bin/bash -x
#SBATCH --account=training2442
#SBATCH --threads-per-core=1
#SBATCH --nodes=1
#SBATCH --ntasks-per-node=4
#SBATCH --cpus-per-task=32
#SBATCH --time=08:00:00
#SBATCH --partition=batch

export RAYON_NUM_THREADS=${SLURM_CPUS_PER_TASK}
srun --cpus-per-task=32 --exclusive target/release/XY-Model --run_id 1 --two 16 32 48 64 80 96 112 128
# 144 160 176 192 208 224 240 256
