# Model report for file:///tmp/top-repos-quality-repos-be5i3o8r/pdfkit.git HEAD 55ed6d547381ed258ba79bfec648b3a04ba77a07

### Dump

```json
{'created_at': '2021-08-30 07:56:11',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '20.3 kB',
 'tags': [],
 'uuid': '47b5c278-4bb9-4746-ac93-86417be30720',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-be5i3o8r/pdfkit.git 55ed6d547381ed258ba79bfec648b3a04ba77a07

# javascript
178 rules, avg.len. 8.0
## train
PPCR: 0.980464
### report
macro
{'f1-score': 0.7875126231011134,
 'precision': 0.8163325002284518,
 'recall': 0.7660635559712281,
 'support': 76836}
micro
{'f1-score': 0.9599276380863138,
 'precision': 0.9599276380863138,
 'recall': 0.9599276380863138,
 'support': 76836}
weighted
{'f1-score': 0.9586269221979496,
 'precision': 0.9580852865757193,
 'recall': 0.9599276380863138,
 'support': 76836}
### report_full
macro
{'f1-score': 0.7473885438296393,
 'precision': 0.8163325002284518,
 'recall': 0.7164144034505736,
 'support': 78367}
micro
{'f1-score': 0.950458431860209,
 'precision': 0.9599276380863138,
 'recall': 0.9411742187400309,
 'support': 78367}
weighted
{'f1-score': 0.9467251382264434,
 'precision': 0.957390385134566,
 'recall': 0.9411742187400309,
 'support': 78367}
## test
PPCR: 0.986083
### report
macro
{'f1-score': 0.7956510729684699,
 'precision': 0.8254758254028399,
 'recall': 0.7784773040536443,
 'support': 21611}
micro
{'f1-score': 0.9756142705103882,
 'precision': 0.9756142705103882,
 'recall': 0.9756142705103882,
 'support': 21611}
weighted
{'f1-score': 0.9750118734516987,
 'precision': 0.9750581113565149,
 'recall': 0.9756142705103882,
 'support': 21611}
### report_full
macro
{'f1-score': 0.7552187039688081,
 'precision': 0.8254758254028399,
 'recall': 0.7343390647764955,
 'support': 21916}
micro
{'f1-score': 0.9687779998621545,
 'precision': 0.9756142705103882,
 'recall': 0.9620368680416135,
 'support': 21916}
weighted
{'f1-score': 0.9661138780752033,
 'precision': 0.9744554576913375,
 'recall': 0.9620368680416135,
 'support': 21916}
```

## javascript
### Summary
127 rules, avg.len. 8.0

| | |
|-|-|
|Min support|142|
|Max support|33202|
|Min confidence|0.9237918257713318|
|Max confidence|0.9993159770965576|

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
| 1 | `  •••start_col ≤ 34<br>	∧ -1.reserved = ,<br>	∧ -5.length ≥ 6<br>⇒ y = ⏎<br>Confidence: 0.994. Support: 3319.` |
| 2 | `  -1.reserved = ,<br>	∧ -5.length ≤ 5<br>	∧ +2.reserved = :<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.959. Support: 355.` |
| 3 | `  -1.reserved = ,<br>	∧ -5.length ≤ 5<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 1666.` |
| 4 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.999. Support: 691.` |
| 5 | `  -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ ^2.internal_type = FunctionExpression<br>⇒ y = ⏎<br>Confidence: 0.993. Support: 2337.` |
| 6 | `  -1.reserved not in {,, ;}<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.996. Support: 142.` |
| 7 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, ;}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.965. Support: 1893.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.roles in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 1619.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -4.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.952. Support: 1733.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {LITERAL}<br>	∧ +5.length ≥ 16<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.931. Support: 153.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.internal_type not in {Identifier}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {LITERAL}<br>	∧ +5.length ≤ 15<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 745.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.924. Support: 1245.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 1066.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 940.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 427.` |
| 16 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 269.` |
| 17 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 242.` |
| 18 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 196.` |
| 19 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = let<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 143.` |
| 20 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 769.` |
| 21 | `  -1.diff_offset ≤ 4<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, let, {, }}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 188.` |
| 22 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, let, {, }}<br>	∧ -4.length ≥ 17<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +4.reserved not in {.}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {Program}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 485.` |
| 23 | `  •••start_col ≥ 22<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, let}<br>	∧ -4.length ≤ 16<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {Program}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 176.` |
| 24 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, let, {, }}<br>	∧ -4.length ≤ 16<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {Program}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 33077.` |
| 25 | `  +1.length ≥ 2<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 4871.` |
| 26 | `  -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.983. Support: 1221.` |
| 27 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ +5.length ≤ 15<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 1997.` |
| 28 | `  -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = .<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.968. Support: 2711.` |
| 29 | `  -1.reserved = ,<br>	∧ -5.diff_offset ≥ 11<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.983. Support: 2829.` |
| 30 | `  -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.944. Support: 1257.` |
| 31 | `  -1.reserved not in {(, ;, {}<br>	∧ -3.diff_offset ≥ 21<br>	∧ -3.label in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.937. Support: 972.` |
| 32 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.diff_offset ≤ 20<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = '<br>Confidence: 0.949. Support: 444.` |
| 33 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_offset ≤ 20<br>	∧ +1.length ≥ 9<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 328.` |
| 34 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>⇒ y = '<br>Confidence: 0.977. Support: 1844.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 1258.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {RIGHT}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 269.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 1306.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.999. Support: 685.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.998. Support: 284.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -4.roles in {NUMBER}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +5.length ≥ 13<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.964. Support: 152.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -4.roles not in {NUMBER}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +5.length ≥ 13<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 517.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +5.length ≤ 12<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 26157.` |
| 43 | `  +1.length ≥ 2<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.995. Support: 4950.` |
| 44 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.975. Support: 1174.` |
| 45 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +5.length ≤ 14<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 2010.` |
| 46 | `  -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = .<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.968. Support: 2730.` |
| 47 | `  -1.reserved not in {(, ;}<br>	∧ -2.roles in {LITERAL}<br>	∧ -5.length ≥ 4<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.971. Support: 3297.` |
| 48 | `  -1.reserved = {<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.927. Support: 1312.` |
| 49 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.960. Support: 464.` |
| 50 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.length ≥ 11<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.928. Support: 370.` |
| 51 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.diff_col ≥ 2<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 4901.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -4.roles not in {NUMBER}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +5.length ≥ 13<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 461.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, =, if}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -4.reserved not in {,}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +5.length ≤ 12<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 18778.` |
| 54 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, =, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.reserved not in {,}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +5.length ≤ 12<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 5947.` |
| 55 | `  +1.length ≥ 2<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 4925.` |
| 56 | `  -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.982. Support: 1167.` |
| 57 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ +5.length ≤ 16<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 2071.` |
| 58 | `  -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = .<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.969. Support: 2705.` |
| 59 | `  -1.reserved not in {(, ;}<br>	∧ -2.roles in {LITERAL}<br>	∧ -5.length ≥ 4<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.976. Support: 3376.` |
| 60 | `  -1.reserved = {<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.941. Support: 1297.` |
| 61 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.966. Support: 427.` |
| 62 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 5163.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.999. Support: 731.` |
| 64 | `  -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = .<br>⇒ y = ⏎<br>Confidence: 0.968. Support: 2710.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -4.label not in {<space>}<br>	∧ +1.roles in {LITERAL, STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.949. Support: 1701.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ +1.roles in {LITERAL} and not in {STRING}<br>	∧ +5.length ≤ 15<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 748.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.942. Support: 1063.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 921.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 444.` |
| 70 | `  •••start_col ≤ 5<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎⏎<br>Confidence: 0.963. Support: 147.` |
| 71 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 728.` |
| 72 | `  -1.diff_offset ≤ 4<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, let, {, }}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.928. Support: 160.` |
| 73 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, let, {, }}<br>	∧ -4.length ≥ 17<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {File, VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 464.` |
| 74 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, let, {, }}<br>	∧ -4.length ≤ 16<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ∅<br>Confidence: 0.967. Support: 475.` |
| 75 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, let, {, }}<br>	∧ -4.length ≤ 16<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement, File, Program, VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 31929.` |
| 76 | `  -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.979. Support: 1156.` |
| 77 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ +5.length ≤ 14<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 1991.` |
| 78 | `  -1.reserved = ,<br>	∧ -5.diff_offset ≥ 12<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.989. Support: 2728.` |
| 79 | `  -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.927. Support: 1279.` |
| 80 | `  -1.reserved not in {(, ;, {}<br>	∧ -3.label in {<newline>}<br>	∧ -5.diff_offset ≥ 30<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.946. Support: 1043.` |
| 81 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -5.diff_offset ≤ 29<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = '<br>Confidence: 0.959. Support: 432.` |
| 82 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -5.diff_offset ≤ 29<br>	∧ +1.length ≥ 9<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 272.` |
| 83 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -5.diff_offset ≤ 29<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 4912.` |
| 84 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -4.internal_type = NumericLiteral<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +5.length ≥ 13<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.925. Support: 181.` |
| 85 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -4.internal_type not in {NumericLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +5.length ≥ 13<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 525.` |
| 86 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +5.length ≤ 12<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 18990.` |
| 87 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +5.length ≤ 12<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 6467.` |
| 88 | `  -1.reserved = :<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 1998.` |
| 89 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.981. Support: 1864.` |
| 90 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -4.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.951. Support: 1780.` |
| 91 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.941. Support: 1263.` |
| 92 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.933. Support: 1126.` |
| 93 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>⇒ y = ␣<br>Confidence: 0.924. Support: 892.` |
| 94 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 435.` |
| 95 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 287.` |
| 96 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 212.` |
| 97 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.998. Support: 209.` |
| 98 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 732.` |
| 99 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, }}<br>	∧ -2.diff_offset ≥ 14<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {CALL}<br>	∧ +5.length ≥ 13<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.997. Support: 150.` |
| 100 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {, }}<br>	∧ -2.diff_offset ≤ 13<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +5.length ≥ 13<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 943.` |
| 101 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {, }}<br>	∧ -2.diff_offset ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +5.length ≤ 12<br>	∧ ^1.internal_type = Program<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 172.` |
| 102 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ -5.diff_offset ≤ 14<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ +5.length ≤ 12<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^1.roles not in {IF, OPERATOR}<br>	∧ ^2.internal_type not in {Program, VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 151.` |
| 103 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {, }}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +5.length ≤ 12<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^1.roles not in {IF, OPERATOR}<br>	∧ ^2.internal_type not in {Program, VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 32184.` |
| 104 | `  -1.reserved not in {(, ;}<br>	∧ -2.roles in {LITERAL}<br>	∧ -5.length ≥ 4<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.971. Support: 3308.` |
| 105 | `  -1.reserved = {<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.930. Support: 1259.` |
| 106 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.957. Support: 476.` |
| 107 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 5029.` |
| 108 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +5.length ≤ 12<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 19009.` |
| 109 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +5.length ≤ 12<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 6695.` |
| 110 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ +5.length ≥ 16<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.938. Support: 153.` |
| 111 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ +5.length ≤ 15<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 2026.` |
| 112 | `  -1.reserved = ,<br>	∧ -5.diff_offset ≥ 12<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.990. Support: 2754.` |
| 113 | `  -1.reserved = ,<br>	∧ -5.diff_offset ≤ 11<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = :<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.958. Support: 201.` |
| 114 | `  -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.941. Support: 1292.` |
| 115 | `  -1.reserved not in {(, ;, {}<br>	∧ -3.label in {<newline>}<br>	∧ -5.diff_offset ≥ 30<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.948. Support: 1053.` |
| 116 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -5.diff_offset ≤ 29<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.970. Support: 386.` |
| 117 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -5.diff_offset ≤ 29<br>	∧ +1.length ≥ 8<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 290.` |
| 118 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.diff_col ≥ 4<br>	∧ -5.diff_offset ≤ 29<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 4867.` |
| 119 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.944. Support: 1131.` |
| 120 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ +1.reserved not in {{, }}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.925. Support: 313.` |
| 121 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = let<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 160.` |
| 122 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 735.` |
| 123 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, }}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +2.reserved = ,<br>	∧ +5.length ≥ 17<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.951. Support: 152.` |
| 124 | `  -1.diff_offset ≤ 4<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, let, {, }}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +5.length ≤ 16<br>	∧ ^1.internal_type = Program<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 173.` |
| 125 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, let, {, }}<br>	∧ -2.roles in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +5.length ≤ 16<br>	∧ ^1.internal_type not in {Program, VariableDeclarator}<br>	∧ ^1.roles in {CALL} and not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {Program}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 162.` |
| 126 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, let, {, }}<br>	∧ -4.length ≥ 17<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +5.length ≤ 16<br>	∧ ^1.internal_type not in {Program, VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {Program}<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 474.` |
| 127 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, let, {, }}<br>	∧ -4.length ≤ 16<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +5.length ≤ 16<br>	∧ ^1.internal_type not in {Program, VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {Program}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 33202.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.031496062992126, "max_conf": 0.9993159770965576, "max_support": 33202, "min_conf": 0.9237918257713318, "min_support": 142, "num_rules": 127}}
```
</details>
