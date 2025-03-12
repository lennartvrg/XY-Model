#!/bin/bash -x
#SBATCH --account=training2442
#SBATCH --threads-per-core=2
#SBATCH --nodes=4
#SBATCH --ntasks-per-node=2
#SBATCH --cpus-per-task=128
#SBATCH --time=06:00:00
#SBATCH --partition=batch

export RAYON_NUM_THREADS=${SLURM_CPUS_PER_TASK}
srun target/release/XY-Model --run_id 1 --one 16 32 48 64 --two 16 32 48 64 80 96 112 128 144 160 176 196 &
