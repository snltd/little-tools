for tool in \
  align-mtimes \
  alsort \
  cf \
  cs \
  flink \
  fseq \
  mixup \
  mmv \
  randos 
do
  cargo install $1 --path $tool
done

