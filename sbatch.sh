#!/bin/bash -x
#SBATCH --account=training2442
#SBATCH --threads-per-core=2
#SBATCH --nodes=4
#SBATCH --ntasks-per-node=2
#SBATCH --cpus-per-task=128
#SBATCH --time=13:30:00
#SBATCH --partition=batch

# *** start of job script ***
# Note: The current working directory at this point is
# the directory where sbatch was executed.

export RAYON_NUM_THREADS=${SLURM_CPUS_PER_TASK}
for size in 8 16 24 32; do
    target/release/XY-Model --run_id 1 --two $size
done