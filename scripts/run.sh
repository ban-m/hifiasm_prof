#!/bin/bash
set -ue
OUTPUT=${PWD}/data
GENOMESIZE=5000000
DIVRATE=0.0001
COVERAGE=40
READLEN=20000
STDDEV=1000
RESULT=${PWD}/result
for i in `seq 0 0`
do
    SEED=${RANDOM}
    OUTDIR=${OUTPUT}/${SEED}
    mkdir -p ${OUTDIR}
    cargo run --release --bin syn_genome -- ${RANDOM} ${OUTDIR} ${GENOMESIZE} ${DIVRATE} \
          ${COVERAGE} ${READLEN} ${STDDEV}
    ASMDIR=${RESULT}/${SEED}
    READS=${OUTDIR}/reads.fa
    mkdir -p ${ASMDIR}
    hifiasm -t56 --h-cov ${COVERAGE} -o ${ASMDIR}/asm ${READS} 2> ${ASMDIR}/log > ${ASMDIR}/out
done

