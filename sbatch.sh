#!/bin/bash -x
#SBATCH --account=training2442
#SBATCH --threads-per-core=2
#SBATCH --nodes=4
#SBATCH --ntasks-per-node=2
#SBATCH --cpus-per-task=128
#SBATCH --time=04:00:00
#SBATCH --partition=batch

export RAYON_NUM_THREADS=${SLURM_CPUS_PER_TASK}

# RUN 1D SIMULATIONS
for size in 16 32 48 64; do
    srun --exclusive -n 128 target/release/XY-Model --run_id 1 --one $size
done

# RUN 2D SIMULATIONS
for size in 16 32 48 64; do
    srun --exclusive -n 128 target/release/XY-Model --run_id 1 --two $size
done
