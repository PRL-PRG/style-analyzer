# Model report for file:///tmp/top-repos-quality-repos-aue3mxdz/leetcode-javascript.git HEAD db6e2ad866e19671305fdad0f117523e91834fdd

### Dump

```json
{'created_at': '2021-08-22 02:48:11',
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
 'size': '18.3 kB',
 'tags': [],
 'uuid': '795c3a4a-a9cc-428e-b802-02559e72eaa2',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-aue3mxdz/leetcode-javascript.git db6e2ad866e19671305fdad0f117523e91834fdd

# javascript
133 rules, avg.len. 7.0
## train
PPCR: 0.963189
### report
macro
{'f1-score': 0.7521090066891779,
 'precision': 0.8036739962980523,
 'recall': 0.7186129230232099,
 'support': 19127}
micro
{'f1-score': 0.9280598107387463,
 'precision': 0.9280598107387463,
 'recall': 0.9280598107387463,
 'support': 19127}
weighted
{'f1-score': 0.9187473704693597,
 'precision': 0.9152884053780221,
 'recall': 0.9280598107387463,
 'support': 19127}
### report_full
macro
{'f1-score': 0.7364147450045532,
 'precision': 0.8036739962980523,
 'recall': 0.6964661063161437,
 'support': 19858}
micro
{'f1-score': 0.9106579453636015,
 'precision': 0.9280598107387463,
 'recall': 0.8938966663309498,
 'support': 19858}
weighted
{'f1-score': 0.8911131380161207,
 'precision': 0.8983347210002206,
 'recall': 0.8938966663309498,
 'support': 19858}
## test
PPCR: 0.966484
### report
macro
{'f1-score': 0.7674296607818859,
 'precision': 0.8120813717352293,
 'recall': 0.7365204240040991,
 'support': 4758}
micro
{'f1-score': 0.9344262295081968,
 'precision': 0.9344262295081968,
 'recall': 0.9344262295081968,
 'support': 4758}
weighted
{'f1-score': 0.9256689961404406,
 'precision': 0.92150222292735,
 'recall': 0.9344262295081968,
 'support': 4758}
### report_full
macro
{'f1-score': 0.7520571238500681,
 'precision': 0.8120813717352293,
 'recall': 0.714940495458296,
 'support': 4923}
micro
{'f1-score': 0.9185001549426711,
 'precision': 0.9344262295081968,
 'recall': 0.903107861060329,
 'support': 4923}
weighted
{'f1-score': 0.9001217545439282,
 'precision': 0.9056804353558179,
 'recall': 0.903107861060329,
 'support': 4923}
```

## javascript
### Summary
105 rules, avg.len. 6.5

| | |
|-|-|
|Min support|130|
|Max support|4151|
|Min confidence|0.9229248762130737|
|Max confidence|0.999367892742157|

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
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 1861.` |
| 2 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 227.` |
| 3 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 589.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 148.` |
| 5 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.length ≤ 13<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 1264.` |
| 6 | `  +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 902.` |
| 7 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 431.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.996. Support: 397.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 611.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 2608.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 684.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, =, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.990. Support: 240.` |
| 13 | `  •••start_col ≥ 16<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = var<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +4.length ≤ 7<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.923. Support: 253.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 373.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, =, var, {, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 385.` |
| 16 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.970. Support: 1870.` |
| 17 | `  -1.reserved = (<br>	∧ +1.roles in {EXPRESSION, STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.998. Support: 227.` |
| 18 | `  -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 557.` |
| 19 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.997. Support: 152.` |
| 20 | `  •••start_col ≥ 24<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -3.diff_offset ≤ 33<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved = ,<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 168.` |
| 21 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -3.diff_offset ≤ 33<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved not in {,}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 2063.` |
| 22 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved not in {,}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 242.` |
| 23 | `  +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 975.` |
| 24 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.999. Support: 425.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.994. Support: 421.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 616.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 2598.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 697.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, =, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.986. Support: 245.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 340.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, =, var, {, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 377.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 233.` |
| 33 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.997. Support: 164.` |
| 34 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.roles in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 33<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved = ,<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 165.` |
| 35 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.998. Support: 244.` |
| 36 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 628.` |
| 37 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 149.` |
| 38 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.roles in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 33<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved = ,<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 142.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 365.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, =, var, {, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 384.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, =, var, {, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.988. Support: 130.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_offset ≤ 9<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 144.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_col ≤ 3<br>	∧ -5.diff_offset ≤ 9<br>	∧ +1.reserved not in {,, ;, =, var, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 151.` |
| 44 | `  -1.roles in {LITERAL, NUMBER}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 237.` |
| 45 | `  -1.roles not in {LITERAL}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1056.` |
| 46 | `  +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.994. Support: 385.` |
| 47 | `  -1.internal_type = Identifier<br>	∧ -1.roles in {CALL}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 704.` |
| 48 | `  -1.reserved = (<br>	∧ -1.roles not in {CALL}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.998. Support: 227.` |
| 49 | `  -1.reserved = (<br>	∧ -1.roles not in {CALL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;, }}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 791.` |
| 50 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {CALL}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ForStatement<br>⇒ y = ␣<br>Confidence: 0.997. Support: 143.` |
| 51 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {CALL}<br>	∧ -2.roles in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 33<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved = ,<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 168.` |
| 52 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {CALL}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 868.` |
| 53 | `  -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION, STRING} and not in {CALL}<br>	∧ +1.reserved not in {;, =, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {BINARY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.997. Support: 162.` |
| 54 | `  -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION} and not in {CALL, STRING}<br>	∧ +1.reserved not in {;, =, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {BINARY}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 1265.` |
| 55 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {CALL, EXPRESSION}<br>	∧ +1.reserved not in {;, =, var, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +2.roles not in {BINARY}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 363.` |
| 56 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {CALL, EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {COMMENT, EXPRESSION}<br>	∧ +2.roles not in {BINARY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 308.` |
| 57 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 157.` |
| 58 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -3.diff_offset ≤ 33<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved = ,<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 162.` |
| 59 | `  -1.roles in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.999. Support: 417.` |
| 60 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.991. Support: 385.` |
| 61 | `  -1.roles not in {STRING}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 666.` |
| 62 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 2650.` |
| 63 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.991. Support: 283.` |
| 64 | `  •••start_col ≥ 16<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = var<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +4.length ≤ 7<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.930. Support: 278.` |
| 65 | `  -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 333.` |
| 66 | `  •••start_col ≤ 18<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 212.` |
| 67 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 1867.` |
| 68 | `  -1.reserved = (<br>	∧ +1.roles in {EXPRESSION, STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 186.` |
| 69 | `  -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 587.` |
| 70 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ForStatement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 158.` |
| 71 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -3.diff_offset ≤ 33<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved = ,<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 181.` |
| 72 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -3.diff_offset ≤ 33<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved not in {,}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 2008.` |
| 73 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved not in {,}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 247.` |
| 74 | `  +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 930.` |
| 75 | `  -1.roles in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 452.` |
| 76 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.999. Support: 383.` |
| 77 | `  -1.roles not in {STRING}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 621.` |
| 78 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 2588.` |
| 79 | `  -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 383.` |
| 80 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.989. Support: 231.` |
| 81 | `  -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -3.diff_offset ≥ 10<br>	∧ +1.reserved not in {var, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved = (<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 142.` |
| 82 | `  •••start_col ≤ 18<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 176.` |
| 83 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 227.` |
| 84 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 603.` |
| 85 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 147.` |
| 86 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 446.` |
| 87 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.999. Support: 345.` |
| 88 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 623.` |
| 89 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 2718.` |
| 90 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 710.` |
| 91 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 377.` |
| 92 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.993. Support: 212.` |
| 93 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, =, var, {, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 355.` |
| 94 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 193.` |
| 95 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {BODY}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 150.` |
| 96 | `  -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.994. Support: 250.` |
| 97 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 644.` |
| 98 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 352.` |
| 99 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.998. Support: 246.` |
| 100 | `  •••start_col ≥ 13<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 4151.` |
| 101 | `  •••start_col ≤ 12<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 139.` |
| 102 | `  •••start_col ≤ 12<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_offset ≤ 6<br>	∧ +1.reserved not in {(, =, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 417.` |
| 103 | `  -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 663.` |
| 104 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {;, =, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.980. Support: 220.` |
| 105 | `  -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.diff_offset ≥ 7<br>	∧ +1.reserved not in {;, var, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 167.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.485714285714286, "max_conf": 0.999367892742157, "max_support": 4151, "min_conf": 0.9229248762130737, "min_support": 130, "num_rules": 105}}
```
</details>
