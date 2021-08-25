# Model report for file:///tmp/top-repos-quality-repos-aodm9vx5/hospital-management-system.git HEAD 580830e297d4dc7afd434653de86d8fe2e9c3d84

### Dump

```json
{'created_at': '2021-08-21 18:58:37',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '20.6 kB',
 'tags': [],
 'uuid': 'a96c5feb-af74-4262-9f71-90aac62725a5',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-aodm9vx5/hospital-management-system.git 580830e297d4dc7afd434653de86d8fe2e9c3d84

# javascript
147 rules, avg.len. 7.2
## train
PPCR: 0.941930
### report
macro
{'f1-score': 0.5924288109812237,
 'precision': 0.665588963342285,
 'recall': 0.5574031366298369,
 'support': 22709}
micro
{'f1-score': 0.8854639129860407,
 'precision': 0.8854639129860408,
 'recall': 0.8854639129860408,
 'support': 22709}
weighted
{'f1-score': 0.8750312455006061,
 'precision': 0.874665696572879,
 'recall': 0.8854639129860408,
 'support': 22709}
### report_full
macro
{'f1-score': 0.5685903689699132,
 'precision': 0.665588963342285,
 'recall': 0.5218691677521439,
 'support': 24109}
micro
{'f1-score': 0.8589858601392628,
 'precision': 0.8854639129860408,
 'recall': 0.8340453772450123,
 'support': 24109}
weighted
{'f1-score': 0.8452482366684086,
 'precision': 0.870488207553078,
 'recall': 0.8340453772450123,
 'support': 24109}
## test
PPCR: 0.898029
### report
macro
{'f1-score': 0.5748675191255017,
 'precision': 0.6146517427695041,
 'recall': 0.5535683564887532,
 'support': 5284}
micro
{'f1-score': 0.8436790310370931,
 'precision': 0.8436790310370931,
 'recall': 0.8436790310370931,
 'support': 5284}
weighted
{'f1-score': 0.8317720177726545,
 'precision': 0.8253033356588644,
 'recall': 0.8436790310370931,
 'support': 5284}
### report_full
macro
{'f1-score': 0.5502232437594602,
 'precision': 0.6146517427695041,
 'recall': 0.5107501138482491,
 'support': 5884}
micro
{'f1-score': 0.798352435530086,
 'precision': 0.8436790310370931,
 'recall': 0.7576478585995922,
 'support': 5884}
weighted
{'f1-score': 0.7853666534409202,
 'precision': 0.8204212151323967,
 'recall': 0.7576478585995922,
 'support': 5884}
```

## javascript
### Summary
75 rules, avg.len. 6.2

| | |
|-|-|
|Min support|135|
|Max support|7071|
|Min confidence|0.9214285612106323|
|Max confidence|0.9996921420097351|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1539.` |
| 2 | `  +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 861.` |
| 3 | `  -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 276.` |
| 4 | `  -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.981. Support: 398.` |
| 5 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 353.` |
| 6 | `  -1.reserved not in {(}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 310.` |
| 7 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.992. Support: 198.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 344.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 145.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 359.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 6492.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.955. Support: 3718.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 927.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 318.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.internal_type = JSXIdentifier<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 214.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 333.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 158.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = JSXAttribute<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 287.` |
| 19 | `  -1.roles in {STRING}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = "<br>Confidence: 0.961. Support: 165.` |
| 20 | `  -1.roles not in {STRING}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.964. Support: 3856.` |
| 21 | `  -1.reserved = .<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 822.` |
| 22 | `  -1.reserved not in {.}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 296.` |
| 23 | `  -1.reserved not in {.}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.996. Support: 381.` |
| 24 | `  -1.reserved not in {.}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 288.` |
| 25 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 331.` |
| 26 | `  -1.reserved = {<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 208.` |
| 27 | `  -1.reserved not in {(, ., {}<br>	∧ -1.roles in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = "<br>Confidence: 0.965. Support: 214.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 326.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {INCOMPLETE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 135.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 148.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type = JSXOpeningElement<br>⇒ y = ∅<br>Confidence: 0.980. Support: 320.` |
| 32 | `  +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 872.` |
| 33 | `  -4.length ≥ 2<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.984. Support: 344.` |
| 34 | `  +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 322.` |
| 35 | `  -1.reserved not in {(}<br>	∧ -2.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT, EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.993. Support: 208.` |
| 36 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.988. Support: 298.` |
| 37 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.921. Support: 350.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 3098.` |
| 39 | `  +1.length ≥ 2<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.997. Support: 832.` |
| 40 | `  -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 244.` |
| 41 | `  -2.label not in {<space>}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.986. Support: 478.` |
| 42 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 361.` |
| 43 | `  -1.reserved not in {(}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 294.` |
| 44 | `  -1.reserved not in {(}<br>	∧ -2.reserved = =<br>	∧ +1.internal_type = JSXIdentifier<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = "<br>Confidence: 0.997. Support: 190.` |
| 45 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved = =<br>⇒ y = "<br>Confidence: 0.996. Support: 396.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1624.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.995. Support: 865.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.926. Support: 386.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 305.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 203.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {KEY} and not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.936. Support: 401.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.927. Support: 7071.` |
| 53 | `  -1.internal_type = StringLiteral<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.988. Support: 456.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = .<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 776.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {.}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 275.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {.}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.984. Support: 407.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {.}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 318.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 359.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 216.` |
| 60 | `  •••start_col ≥ 27<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ., {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {KEY} and not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ⏎<br>Confidence: 0.925. Support: 247.` |
| 61 | `  -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 254.` |
| 62 | `  -1.internal_type = StringLiteral<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.978. Support: 247.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.length ≥ 2<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.993. Support: 368.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 297.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {CONDITION, DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 339.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 6930.` |
| 67 | `  -2.label not in {<space>}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.979. Support: 404.` |
| 68 | `  -1.reserved not in {(}<br>	∧ -1.roles in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.986. Support: 242.` |
| 69 | `  -1.reserved = {<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.internal_type = JSXIdentifier<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 200.` |
| 70 | `  •••start_col ≥ 27<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER, LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {KEY} and not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.954. Support: 229.` |
| 71 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles in {EXPRESSION} and not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 177.` |
| 72 | `  -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, IF}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 331.` |
| 73 | `  -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, IF}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 301.` |
| 74 | `  -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, IF}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 237.` |
| 75 | `  -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, IF}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 230.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.213333333333333, "max_conf": 0.9996921420097351, "max_support": 7071, "min_conf": 0.9214285612106323, "min_support": 135, "num_rules": 75}}
```
</details>
