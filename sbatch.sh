#!/bin/bash -x
#SBATCH --account=training2442
#SBATCH --threads-per-core=2
#SBATCH --nodes=3
#SBATCH --ntasks-per-node=2
#SBATCH --cpus-per-task=128
#SBATCH --time=04:00:00
#SBATCH --partition=batch

export RAYON_NUM_THREADS=${SLURM_CPUS_PER_TASK}
srun --cpus-per-task=128 --cpu-bind=sockets --overcommit --exclusive target/release/XY-Model --run_id 1 --one 16 32 48 64 --two 16 32 48 64 80 96 112 128
