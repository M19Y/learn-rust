find . -type f -name "*.rs" -not -path "./target/*" -print0 |
  while IFS= read -r -d '' file; do
    dir=$(dirname "$file")
    name=$(basename "$file")

    new_name=$(echo "$name" | sed 's/-/_/g')

    if [ "$name" != "$new_name" ]; then
      mv -- "$file" "$dir/$new_name"
      echo "$name -> $name_name"
    fi
  done
