// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{Expression, Span};
use leo_grammar::console::FormattedParameter as GrammarFormattedParameter;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormattedParameter {
    pub expression: Expression,
    pub span: Span,
}

impl<'ast> From<GrammarFormattedParameter<'ast>> for FormattedParameter {
    fn from(parameter: GrammarFormattedParameter<'ast>) -> Self {
        Self {
            expression: Expression::from(parameter.expression),
            span: Span::from(parameter.span),
        }
    }
}

impl fmt::Display for FormattedParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.expression)
    }
}
