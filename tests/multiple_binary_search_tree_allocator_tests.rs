#![feature(allocator_api)]

#[cfg(test)]
mod multiple_binary_search_tree_allocator_tests {
    use allocator_suite::prelude::*;
    use allocator_suite::extensions::usize_ext::UsizeExt;
    use allocator_suite::allocators::prelude::*;
    use allocator_suite::allocators::prelude::*;
    use allocator_suite::prelude::mmap::prelude::MemoryMapSource;
    use std::alloc::AllocErr;
    use allocator_suite::allocators::binary_search_trees::binary_search_trees_with_cached_knowledge_of_first_child::BinarySearchTreesWithCachedKnowledgeOfFirstChild;
    use allocator_suite::extensions::non_null_u8_ext::NonNullU8Ext;

    #[test]
    pub fn repeated_small_allocations() {
        test_repeated_small_allocations(32);
        test_repeated_small_allocations(64);
        test_repeated_small_allocations(96);
        test_repeated_small_allocations(128);
        test_repeated_small_allocations(160);
        test_repeated_small_allocations(192);
        test_repeated_small_allocations(256);
    }

    #[test]
    pub fn mixed_allocations() {
        let allocator = new_allocator(256);

        allocator
            .allocate(32.non_zero(), 8.non_zero())
            .expect(&format!("Did not allocate"));
        allocator
            .allocate(128.non_zero(), 8.non_zero())
            .expect(&format!("Did not allocate"));
        allocator
            .allocate(64.non_zero(), 8.non_zero())
            .expect(&format!("Did not allocate"));
        allocator
            .allocate(32.non_zero(), 8.non_zero())
            .expect(&format!("Did not allocate"));
        assert_allocator_is_empty(&allocator);
    }

    #[test]
    pub fn shrink_allocation_within_block() {
        const ALLOCATION_SIZE: usize = 32;
        const MEMORY_PATTERN: [u8; ALLOCATION_SIZE] = [0x0A; ALLOCATION_SIZE];

        let allocator = new_allocator(256);

        let allocation = allocator
            .allocate(ALLOCATION_SIZE.non_zero(), 8.non_zero())
            .expect(&format!("Did not allocate"));
        allocation.write(MEMORY_PATTERN);

        let reallocation = allocator
            .shrinking_reallocate(
                16.non_zero(),
                8.non_zero(),
                ALLOCATION_SIZE.non_zero(),
                allocation,
            )
            .expect(&format!("Did not reallocate"));
        assert_eq!(
            allocation, reallocation,
            "Did not shrink allocation within block"
        );
        assert_eq!(
            reallocation.read::<[u8; ALLOCATION_SIZE]>(),
            MEMORY_PATTERN,
            "Did not preserve memory contents when shrinking block"
        );
    }

    #[test]
    pub fn shrink_allocation_within_block_and_deallocate_unused_block() {
        let allocator = new_allocator(64);

        let original_allocation = allocator
            .allocate(64.non_zero(), 8.non_zero())
            .expect(&format!("Did not allocate"));
        assert_allocator_is_empty(&allocator);

        let reallocation = allocator
            .shrinking_reallocate(
                32.non_zero(),
                8.non_zero(),
                64.non_zero(),
                original_allocation,
            )
            .expect(&format!("Did not reallocate"));
        assert_eq!(
            original_allocation, reallocation,
            "Did not shrink allocation within block"
        );

        let _allocation = allocator
            .allocate(32.non_zero(), 8.non_zero())
            .expect(&format!("Did not allocate recently freed block"));
        assert_allocator_is_empty(&allocator);
    }

    #[test]
    pub fn grow_allocation_into_larger_block() {
        const ALLOCATION_SIZE: usize = 32;
        const MEMORY_PATTERN: [u8; ALLOCATION_SIZE] = [0x0A; ALLOCATION_SIZE];

        let allocator = new_allocator(64);

        let allocation = allocator
            .allocate(ALLOCATION_SIZE.non_zero(), 8.non_zero())
            .expect(&format!("Did not allocate"));
        allocation.write(MEMORY_PATTERN);

        let reallocation = allocator
            .growing_reallocate(
                (ALLOCATION_SIZE + 1).non_zero(),
                8.non_zero(),
                ALLOCATION_SIZE.non_zero(),
                allocation,
            )
            .expect(&format!("Did not reallocate"));
        assert_eq!(
            allocation, reallocation,
            "Did not shrink allocation within block"
        );
        assert_eq!(
            reallocation.read::<[u8; ALLOCATION_SIZE]>(),
            MEMORY_PATTERN,
            "Did not preserve memory contents when growing block"
        );
    }

    #[test]
    pub fn deallocation() {
        const ALLOCATION_SIZE: usize = 31;

        let allocator = new_allocator(32);
        let allocation = allocator
            .allocate(ALLOCATION_SIZE.non_zero(), 8.non_zero())
            .expect(&format!("Did not allocate"));
        assert_allocator_is_empty(&allocator);

        allocator.deallocate(ALLOCATION_SIZE.non_zero(), 8.non_zero(), allocation);
        let _allocation = allocator
            .allocate(ALLOCATION_SIZE.non_zero(), 8.non_zero())
            .expect(&format!("Did not allocate"));
        assert_allocator_is_empty(&allocator);
    }

    fn test_repeated_small_allocations(memory_size: usize) {
        let allocator = new_allocator(memory_size);

        for allocation_loop_count in 0..memory_size / SMALLEST_ALLOCATION {
            let _ = allocator
                .allocate(1.non_zero(), 1.non_zero())
                .expect(&format!(
                    "Did not allocate for loop `{}`",
                    allocation_loop_count
                ));
        }
        assert_allocator_is_empty(&allocator);
    }

    fn assert_allocator_is_empty(allocator: &MultipleBinarySearchTreeAllocator<MemoryMapSource>) {
        assert_eq!(
            allocator.allocate(1.non_zero(), 1.non_zero()),
            Err(AllocErr),
            "Allocator was not empty"
        );
    }

    fn new_allocator<'a>(memory_size: usize) -> MultipleBinarySearchTreeAllocator<MemoryMapSource> {
        let memory_source = MemoryMapSource::default();
        let allocator =
            MultipleBinarySearchTreeAllocator::new(memory_source, memory_size.non_zero()).unwrap();
        allocator
    }

    const SMALLEST_ALLOCATION: usize =
        BinarySearchTreesWithCachedKnowledgeOfFirstChild::MINIMUM_ALLOCATION_SIZE.get();
}
