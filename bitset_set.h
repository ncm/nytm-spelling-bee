#ifndef BITSET_SET_H
#define BITSET_SET_H

#include <bitset>
#include <iterator>

namespace tst {

template <std::size_t N>
struct bitset_set : public std::bitset<N> {
    using Base = std::bitset<N>;
    bitset_set() {} 
    bitset_set(unsigned long bits) : Base{bits} {} 
    bitset_set(std::bitset<N> bits) : Base{bits} {} 
    struct iterator_type
            : std::iterator<std::forward_iterator_tag,Base,int> {
        Base rest;
        iterator_type() {}
        iterator_type(bitset_set bits) : rest{bits} {}
        iterator_type& operator++() {
                rest &= ~-rest.to_ulong(); return *this; }
        iterator_type operator++(int) {
                iterator_type tmp = rest; ++*this; return tmp; }
        bitset_set operator*() const {
                return bitset_set{rest & Base{-rest.to_ulong()}}; }
        bool operator!=(iterator_type o) const { return rest != o.rest; }
        bool operator==(iterator_type o) const { return rest == o.rest; }
    };
    iterator_type begin() const { return iterator_type{*this}; }
    iterator_type end() const { return iterator_type{}; }
    std::size_t least_bit_position() {
        unsigned long bits = this->to_ulong();
        return std::bitset<N>((bits & -bits)-1).count(); }
    std::size_t size() { return this->count(); }
};

template <std::size_t N>
bool operator<(bitset_set<N> a, bitset_set<N> b) {
    return a.to_ulong() < b.to_ulong(); }

template <std::size_t N>
bool operator>(bitset_set<N> a, bitset_set<N> b) {
    return a.to_ulong() > b.to_ulong(); }

}

#endif
