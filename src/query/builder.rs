use query::Query;
use query::Modifier;
use query::Direction;
use query::Equality;
use query::SqlType;
use query::Operand;
use query::{ColumnName,ToColumnName};
use query::NullsWhere;
use query::{Join,JoinType};
use query::{TableName,ToTableName};
use query::Filter;
use dao::{Value,ToValue};
use query::Field;
use query::Order;
use query::operand::ToOperand;
use query::order::{ToOrder,HasDirection};
use database::Database;
use writer::SqlFrag;
use database::BuildMode;
use query::join::ToJoin;
use dao::IsDao;
use table::IsTable;
use database::DbError;

pub struct QueryBuilder{
    query: Query
}


 pub fn SELECT_ALL() ->QueryBuilder {
 	QueryBuilder::SELECT_ALL()
 }

 pub fn SELECT()->QueryBuilder {
 	QueryBuilder::SELECT()
 }

 pub fn INSERT()->QueryBuilder {
 	QueryBuilder::INSERT()
 }

 pub fn UPDATE()->QueryBuilder {
 	QueryBuilder::UPDATE()
 }

 pub fn DELETE()->QueryBuilder {
 	QueryBuilder::DELETE()
 }

impl QueryBuilder {

    pub fn SELECT() -> Self {
        let mut q = Query::new();
        q.sql_type = SqlType::SELECT;
        QueryBuilder{query: q}
    }

    pub fn INSERT() -> Self {
        let mut q = Query::new();
        q.sql_type = SqlType::INSERT;
        QueryBuilder{query: q}
    }
    pub fn UPDATE() -> Self {
        let mut q = Query::new();
        q.sql_type = SqlType::UPDATE;
        QueryBuilder{query: q}
    }
    pub fn DELETE() -> Self {
        let mut q = Query::new();
        q.sql_type = SqlType::DELETE;
        QueryBuilder{query: q}
    }

    /// add DISTINCT ie: SELECT DISTINCT
    pub fn DISTINCT(&mut self) -> &mut Self {
        self.query.distinct = true;
        self
    }

    pub fn SELECT_ALL() ->Self {
        let mut qb = Self::SELECT();
		qb.ALL();
		qb
    }

    pub fn ALL(&mut self) -> &mut Self {
        self.query.all();
        self
    }

    pub fn GROUP_BY(&mut self, to_operand: &ToOperand) -> &mut Self {
		self.query.group_by.push(to_operand.to_operand());
        self
    }

    pub fn HAVING(&mut self, filter: Filter) -> &mut Self {
		self.query.having.push(filter);
        self
    }

    
    pub fn LIMIT(&mut self, n: usize)->&mut Self{
        self.query.set_page_size(n);
        self
    }

    /// A more terse way to write the query
    /// only 1 table is supported yet
    pub fn FROM(&mut self, table: &ToTableName) -> &mut Self {
        let table_name = table.to_table_name();
        let operand = Operand::TableName(table_name);
        let field = Field {
            operand: operand,
            name: None,
        };
        self.query.from_field(field);
        self
    } 

    
    /// `into` is used in rust, os settled with `into_`
    pub fn INTO(&mut self, table: &ToTableName) -> &mut Self {
        assert_eq!(self.query.sql_type, SqlType::INSERT);
        self.FROM(table);
        self
    }
    /// can be used in behalf of into_, from,
    pub fn TABLE(&mut self, table: &ToTableName) -> &mut Self {
        self.FROM(table);
        self
    }

    /// if the database support CTE declareted query i.e WITH,
    /// then this query will be declared
    /// if database doesn't support WITH queries, then this query will be
    /// wrapped in the from_query
    /// build a builder for this
    pub fn WITH(&mut self, query: Query, alias: &str) -> &mut Self {
        self.query.declared_query.insert(alias.to_owned(), query);
        self
    }


    /// join a table on this query
    ///
    pub fn JOIN(&mut self, join: Join) -> &mut Self {
        self.query.joins.push(join);
        self
    }

    /// join a table on this query
    ///
    pub fn LEFT_JOIN(&mut self, join: Join) -> &mut Self {
		let mut join = join.clone();
		join.modifier = Some(Modifier::LEFT);
		self.JOIN(join);
        self
    }
    pub fn RIGHT_JOIN(&mut self, join: Join) -> &mut Self {
		let mut join = join.clone();
		join.modifier = Some(Modifier::RIGHT);
		self.JOIN(join);
        self
    }
    pub fn FULL_JOIN(&mut self, join: Join) -> &mut Self {
		let mut join = join.clone();
		join.modifier = Some(Modifier::FULL);
		self.JOIN(join);
        self
    }
    pub fn INNER_JOIN(&mut self, join: Join) -> &mut Self {
		let mut join = join.clone();
		join.join_type = Some(JoinType::INNER);
		self.JOIN(join);
        self
    }
    
	pub fn WHERE(&mut self, filter: Filter) -> &mut Self {
		self.query.filters.push(filter);
        self
    }

	pub fn ORDER_BY(&mut self, to_order: &ToOrder)->&mut Self {
		let mut orders = to_order.to_order();
		self.query.order_by.append(&mut orders);
        self
	}



    /// build the query only, not executed, useful when debugging
    pub fn build(&mut self, db: &Database) -> SqlFrag {
        self.query.build(db)
    }

    pub fn collect_one<T: IsDao + IsTable>(&mut self, db: &Database) -> Result<T, DbError> {
		self.query.collect_one::<T>(db)
	}
    pub fn collect<T: IsDao + IsTable>(&mut self, db: &Database) -> Result<Vec<T>, DbError> {
		self.query.collect::<T>(db)
	}
}