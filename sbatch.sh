#!/bin/bash -x
#SBATCH --account=training2442
#SBATCH --threads-per-core=2
#SBATCH --nodes=1
#SBATCH --ntasks-per-node=1
#SBATCH --cpus-per-task=256
#SBATCH --time=13:30:00
#SBATCH --partition=batch

# *** start of job script ***
# Note: The current working directory at this point is
# the directory where sbatch was executed.

export RAYON_NUM_THREADS=${SLURM_CPUS_PER_TASK}
srun target/release/XY-Model --two 176 192 256 272 288 304 320