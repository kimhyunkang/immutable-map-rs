var searchIndex = {};
searchIndex["immutable_map"] = {"doc":"Immutable binary search tree.","items":[[4,"Bound","immutable_map","An endpoint of a range of keys.",null,null],[13,"Unbounded","","An infinite endpoint. Indicates that there is no bound in this direction.",0,null],[13,"Included","","An inclusive bound.",0,null],[13,"Excluded","","An exclusive bound.",0,null],[0,"set","","An immutable set based on binary search tree",null,null],[3,"TreeSet","immutable_map::set","An immutable set based on weight-balanced binary tree. See https://yoichihirai.com/bst.pdf for the balancing algorithm.",null,null],[3,"Intersection","","",null,null],[3,"Union","","",null,null],[3,"Difference","","",null,null],[3,"SymmetricDifference","","",null,null],[6,"TreeSetIter","","",null,null],[6,"TreeSetRevIter","","",null,null],[6,"TreeSetRange","","",null,null],[11,"clone","","",1,{"inputs":[{"name":"self"}],"output":{"name":"treeset"}}],[11,"default","","",1,{"inputs":[],"output":{"name":"treeset"}}],[11,"new","","Makes a new empty TreeSet",1,{"inputs":[],"output":{"name":"treeset"}}],[11,"len","","Returns the number of elements in the set.",1,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"is_empty","","Returns true if the set contains no elements.",1,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"iter","","Gets an iterator over the entries of the set, in sorted order.",1,{"inputs":[{"name":"self"}],"output":{"name":"treesetiter"}}],[11,"rev_iter","","Gets an iterator over the entries of the set, in decreasing order.",1,{"inputs":[{"name":"self"}],"output":{"name":"treesetreviter"}}],[11,"get","","Returns a reference to the value in the set, if any, that is equal to the given value.",1,{"inputs":[{"name":"self"},{"name":"q"}],"output":{"name":"option"}}],[11,"contains","","Returns true if the value is in the set, if any, that is equal to the given value.",1,{"inputs":[{"name":"self"},{"name":"q"}],"output":{"name":"bool"}}],[11,"range","","Constructs a double-ended iterator over a sub-range of elements in the set, starting at min, and ending at max. If min is Unbounded, then it will be treated as \"negative infinity\", and if max is Unbounded, then it will be treated as \"positive infinity\". Thus range(Unbounded, Unbounded) will yield the whole collection.",1,{"inputs":[{"name":"self"},{"name":"bound"},{"name":"bound"}],"output":{"name":"treesetrange"}}],[11,"intersection","","Visits the values representing the intersection, in ascending order.",1,{"inputs":[{"name":"self"},{"name":"treeset"}],"output":{"name":"intersection"}}],[11,"union","","Visits the values representing the union, in ascending order.",1,{"inputs":[{"name":"self"},{"name":"treeset"}],"output":{"name":"union"}}],[11,"difference","","Visits the values representing the difference of `self` and `other`, in ascending order.",1,{"inputs":[{"name":"self"},{"name":"treeset"}],"output":{"name":"difference"}}],[11,"symmetric_difference","","Visits the values representing the symmetric difference, in ascending order.",1,{"inputs":[{"name":"self"},{"name":"treeset"}],"output":{"name":"symmetricdifference"}}],[11,"is_disjoint","","Returns true if the set has no elements in common with other. This is equivalent to checking for an empty intersection.",1,{"inputs":[{"name":"self"},{"name":"treeset"}],"output":{"name":"bool"}}],[11,"is_subset","","Returns true if `self` is a subset of `other`.",1,{"inputs":[{"name":"self"},{"name":"treeset"}],"output":{"name":"bool"}}],[11,"is_superset","","Returns true if `self` is a superset of `other`.",1,{"inputs":[{"name":"self"},{"name":"treeset"}],"output":{"name":"bool"}}],[11,"insert","","Returns a new set with the value added to the set, replacing the existing value, if any.",1,{"inputs":[{"name":"self"},{"name":"v"}],"output":{"name":"treeset"}}],[11,"insert_if_absent","","Return a new copy of `TreeSet` with the value inserted.",1,{"inputs":[{"name":"self"},{"name":"v"}],"output":{"name":"option"}}],[11,"delete_min","","Returns a new set with the smallest element removed from the set, and the smallest element. Returns `None` if the set was empty",1,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"delete_max","","Returns a new set with the largest element removed from the set, and the largest element. Returns `None` if the set was empty",1,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"remove","","Returns the new set with the value removed, and the removed value",1,{"inputs":[{"name":"self"},{"name":"q"}],"output":{"name":"option"}}],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",1,{"inputs":[{"name":"self"},{"name":"treeset"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",1,{"inputs":[{"name":"self"},{"name":"treeset"}],"output":{"name":"option"}}],[11,"cmp","","",1,{"inputs":[{"name":"self"},{"name":"treeset"}],"output":{"name":"ordering"}}],[11,"from_iter","","",1,{"inputs":[{"name":"t"}],"output":{"name":"treeset"}}],[11,"clone","","",2,{"inputs":[{"name":"self"}],"output":{"name":"intersection"}}],[11,"next","","",2,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"clone","","",3,{"inputs":[{"name":"self"}],"output":{"name":"union"}}],[11,"next","","",3,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"clone","","",4,{"inputs":[{"name":"self"}],"output":{"name":"difference"}}],[11,"next","","",4,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"clone","","",5,{"inputs":[{"name":"self"}],"output":{"name":"symmetricdifference"}}],[11,"next","","",5,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[0,"map","immutable_map","An immutable map based on binary search tree",null,null],[3,"TreeMap","immutable_map::map","An immutable key-value map based on weight-balanced binary tree. See https://yoichihirai.com/bst.pdf for the balancing algorithm.",null,null],[6,"TreeMapIter","","",null,null],[6,"TreeMapRevIter","","",null,null],[6,"TreeMapRange","","",null,null],[6,"TreeMapKeys","","",null,null],[6,"TreeMapValues","","",null,null],[11,"clone","","",6,{"inputs":[{"name":"self"}],"output":{"name":"treemap"}}],[11,"default","","",6,{"inputs":[],"output":{"name":"treemap"}}],[11,"new","","Makes a new empty TreeMap",6,{"inputs":[],"output":{"name":"treemap"}}],[11,"len","","Returns the number of elements in the map.",6,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"is_empty","","Returns true if the map contains no elements.",6,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"iter","","Gets an iterator over the entries of the map, sorted by key.",6,{"inputs":[{"name":"self"}],"output":{"name":"treemapiter"}}],[11,"rev_iter","","Gets an iterator over the entries of the map, sorted by key in decreasing order.",6,{"inputs":[{"name":"self"}],"output":{"name":"treemapreviter"}}],[11,"keys","","Gets an iterator over the keys of the map, in increasing order.",6,{"inputs":[{"name":"self"}],"output":{"name":"treemapkeys"}}],[11,"values","","Gets an iterator over the values of the map, ordered by key.",6,{"inputs":[{"name":"self"}],"output":{"name":"treemapvalues"}}],[11,"get","","Returns a reference to the value corresponding to the key.",6,{"inputs":[{"name":"self"},{"name":"q"}],"output":{"name":"option"}}],[11,"contains_key","","Returns true if the map contains given key",6,{"inputs":[{"name":"self"},{"name":"q"}],"output":{"name":"bool"}}],[11,"range","","Constructs a double-ended iterator over a sub-range of elements in the map, starting at min, and ending at max. If min is Unbounded, then it will be treated as \"negative infinity\", and if max is Unbounded, then it will be treated as \"positive infinity\". Thus range(Unbounded, Unbounded) will yield the whole collection.",6,{"inputs":[{"name":"self"},{"name":"bound"},{"name":"bound"}],"output":{"name":"treemaprange"}}],[11,"insert","","Return a new copy of `TreeMap` with the key-value pair inserted",6,{"inputs":[{"name":"self"},{"name":"k"},{"name":"v"}],"output":{"name":"treemap"}}],[11,"insert_if_absent","","Return a new copy of `TreeMap` with the key-value pair inserted.",6,{"inputs":[{"name":"self"},{"name":"k"},{"name":"v"}],"output":{"name":"option"}}],[11,"update","","Find the map with given key, and if the key is found, udpate the value with the provided function `f`, and return the new map. Returns `None` if the map already has the key.",6,{"inputs":[{"name":"self"},{"name":"q"},{"name":"f"}],"output":{"name":"option"}}],[11,"insert_or_update","","Find the map with given key, and if the key is found, udpate the value with the provided function `f`, and return the new map. If the key is not found, insert the key-value pair to the map and return it.",6,{"inputs":[{"name":"self"},{"name":"k"},{"name":"v"},{"name":"f"}],"output":{"name":"treemap"}}],[11,"delete_min","","Remove the smallest key-value pair from the map, and returns the modified copy.",6,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"delete_max","","Remove the largest key-value pair from the map, and returns the modified copy.",6,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"remove","","Remove the key from the map",6,{"inputs":[{"name":"self"},{"name":"q"}],"output":{"name":"option"}}],[11,"fmt","","",6,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",6,{"inputs":[{"name":"self"},{"name":"treemap"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",6,{"inputs":[{"name":"self"},{"name":"treemap"}],"output":{"name":"option"}}],[11,"cmp","","",6,{"inputs":[{"name":"self"},{"name":"treemap"}],"output":{"name":"ordering"}}],[11,"index","","",6,{"inputs":[{"name":"self"},{"name":"q"}],"output":{"name":"v"}}],[11,"from_iter","","",6,{"inputs":[{"name":"t"}],"output":{"name":"treemap"}}],[11,"clone","immutable_map","",0,{"inputs":[{"name":"self"}],"output":{"name":"bound"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",0,null],[11,"eq","","",0,{"inputs":[{"name":"self"},{"name":"bound"}],"output":{"name":"bool"}}],[11,"ne","","",0,{"inputs":[{"name":"self"},{"name":"bound"}],"output":{"name":"bool"}}]],"paths":[[4,"Bound"],[3,"TreeSet"],[3,"Intersection"],[3,"Union"],[3,"Difference"],[3,"SymmetricDifference"],[3,"TreeMap"]]};
initSearch(searchIndex);
