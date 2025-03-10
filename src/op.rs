/* op.rs --- OP

*
* Author: M.R.Siavash Katebzadeh <mr@katebzadeh.xyz>
* Keywords: Rust
* Version: 0.0.1
*
* This program is free software; you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use crate::key::CacheKey;
use std::sync::{atomic::AtomicU64, Arc};

pub(crate) type SeqLock = AtomicU64;

const fn find_padding_cust_align(size: usize, align: usize) -> usize {
    (align - (size % align)) % align
}

const CACHE_LINE: usize = 64;
const VALUE_SIZE_: usize = 32;
const VALUE_SIZE: usize = VALUE_SIZE_ + find_padding_cust_align(VALUE_SIZE_, 64);
const OP_VALUE_SIZE: usize = VALUE_SIZE + find_padding_cust_align(VALUE_SIZE, 32);
const OP_SIZE_: usize = 32 + OP_VALUE_SIZE;
const OP_PADDING_SIZE: usize = find_padding_cust_align(OP_SIZE_, 64);
pub(crate) const OP_SIZE: usize = OP_SIZE_ + OP_PADDING_SIZE;

#[derive(Clone)]
pub(crate) struct Op {
    pub(crate) value: [u8; OP_VALUE_SIZE],
    pub(crate) key: CacheKey,
    pub(crate) seqlock: Arc<SeqLock>,
    pub(crate) version: u64,
    pub(crate) m_id: u8,
    pub(crate) state: u8,
    _unused: [u8; 2],
    pub(crate) key_id: u32,
    _padding: [u8; OP_PADDING_SIZE],
}

impl Op {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl Default for Op {
    fn default() -> Self {
        Self {
            value: [0; OP_VALUE_SIZE],
            key: CacheKey::default(),
            seqlock: Arc::new(SeqLock::new(0)),
            version: 0,
            m_id: 0,
            state: 0,
            _unused: [0, 0],
            key_id: 0,
            _padding: [0; OP_PADDING_SIZE],
        }
    }
}

/* op.rs ends here */
