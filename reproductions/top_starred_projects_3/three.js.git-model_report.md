# Model report for file:///tmp/top-repos-quality-repos-wl6s36k1/three.js.git HEAD 41fe3d8edd05e76139aff5c96a2151efbd48859d

### Dump

```json
{'created_at': '2021-08-30 00:46:06',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.11.0-31-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '28.0 kB',
 'tags': [],
 'uuid': '79da36b4-454f-472e-8d4e-39314b14113f',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-wl6s36k1/three.js.git 41fe3d8edd05e76139aff5c96a2151efbd48859d

# javascript
161 rules, avg.len. 13.1
## train
PPCR: 0.965586
### report
macro
{'f1-score': 0.5966426718871674,
 'precision': 0.6994725972939938,
 'recall': 0.5670589778936753,
 'support': 780824}
micro
{'f1-score': 0.9604443510957655,
 'precision': 0.9604443510957655,
 'recall': 0.9604443510957655,
 'support': 780824}
weighted
{'f1-score': 0.9576155072880757,
 'precision': 0.958604921120995,
 'recall': 0.9604443510957655,
 'support': 780824}
### report_full
macro
{'f1-score': 0.5613578097061521,
 'precision': 0.6994725972939938,
 'recall': 0.5110720198402411,
 'support': 808653}
micro
{'f1-score': 0.9436286275296843,
 'precision': 0.9604443510957655,
 'recall': 0.9273916006000101,
 'support': 808653}
weighted
{'f1-score': 0.9349328071768369,
 'precision': 0.9542408642865863,
 'recall': 0.9273916006000101,
 'support': 808653}
## test
PPCR: 0.963831
### report
macro
{'f1-score': 0.4980586410698363,
 'precision': 0.516689191728263,
 'recall': 0.4937306167204339,
 'support': 182859}
micro
{'f1-score': 0.9639558348235526,
 'precision': 0.9639558348235526,
 'recall': 0.9639558348235526,
 'support': 182859}
weighted
{'f1-score': 0.9613382392784077,
 'precision': 0.9614992732807991,
 'recall': 0.9639558348235526,
 'support': 182859}
### report_full
macro
{'f1-score': 0.4683897379209431,
 'precision': 0.516689191728263,
 'recall': 0.4422207509609486,
 'support': 189721}
micro
{'f1-score': 0.9462021579258145,
 'precision': 0.9639558348235526,
 'recall': 0.9290906120039426,
 'support': 189721}
weighted
{'f1-score': 0.9372755944252789,
 'precision': 0.9558375663310076,
 'recall': 0.9290906120039426,
 'support': 189721}
```

## javascript
### Summary
111 rules, avg.len. 12.8

| | |
|-|-|
|Min support|92|
|Max support|60880|
|Min confidence|0.9253979921340942|
|Max confidence|0.9998201727867126|

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
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
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
| 1 | `  -1.reserved = [<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 7926.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {[}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.995. Support: 329.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {[}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 1515.` |
| 4 | `  -1.reserved not in {[}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 452.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {[}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.949. Support: 227.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {[}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 1377.` |
| 7 | `  -1.reserved not in {[}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {), ]}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.995. Support: 492.` |
| 8 | `  -1.reserved not in {[}<br>	∧ -3.reserved = [<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ]}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 750.` |
| 9 | `  -1.reserved not in {[}<br>	∧ -3.reserved not in {[}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 60880.` |
| 10 | `  -1.roles in {STRING}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.972. Support: 1912.` |
| 11 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 42805.` |
| 12 | `  -1.roles in {STRING}<br>	∧ +1.reserved = ,<br>	∧ +3.roles not in {FUNCTION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.972. Support: 2833.` |
| 13 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 30715.` |
| 14 | `  -1.reserved = ;<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.956. Support: 171.` |
| 15 | `  -1.reserved = ;<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎⇥⁻<br>Confidence: 0.941. Support: 9490.` |
| 16 | `  -1.reserved = ;<br>	∧ +1.reserved not in {,, ;, }}<br>	∧ ^1.internal_type = ForStatement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 2780.` |
| 17 | `  -1.reserved = ;<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {,, ;, }}<br>	∧ +2.reserved = (<br>	∧ +3.reserved = let<br>	∧ ^1.internal_type not in {File, ForStatement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.931. Support: 733.` |
| 18 | `  •••start_col ≥ 12<br>	∧ -1.reserved = ;<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +1.reserved = export<br>	∧ +2.reserved not in {(}<br>	∧ +2.length ≥ 6<br>	∧ ^1.internal_type not in {File, ForStatement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {BlockStatement}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ⏎⏎<br>Confidence: 0.995. Support: 93.` |
| 19 | `  •••start_col ≥ 12<br>	∧ -1.reserved = ;<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +1.reserved = export<br>	∧ +2.reserved not in {(}<br>	∧ +2.length ≤ 6<br>	∧ +5.length ≥ 2<br>	∧ ^1.internal_type not in {File, ForStatement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {BlockStatement}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ⏎<br>Confidence: 0.936. Support: 226.` |
| 20 | `  •••start_col ≥ 12<br>	∧ -1.reserved = ;<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +1.reserved not in {,, ;, return, }}<br>	∧ +2.reserved not in {(}<br>	∧ +3.internal_type not in {NumericLiteral}<br>	∧ ^1.internal_type not in {File, ForStatement}<br>	∧ ^1.roles in {BODY} and not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {BlockStatement}<br>	∧ ^2.roles in {INITIALIZATION} and not in {CALL}<br>⇒ y = ⏎<br>Confidence: 0.968. Support: 421.` |
| 21 | `  •••start_col ≤ 11<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {,, ;, }}<br>	∧ +1.length = 0<br>	∧ +2.reserved not in {(}<br>	∧ ^1.internal_type not in {File, ForStatement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.994. Support: 252.` |
| 22 | `  -1.roles in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 31202.` |
| 23 | `  •••start_col ≥ 5<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.roles not in {KEY}<br>	∧ +1.reserved = (<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 11013.` |
| 24 | `  •••start_col ≤ 4<br>	∧ -1.diff_col ≥ 2<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 134.` |
| 25 | `  •••start_col ≤ 4<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 234.` |
| 26 | `  •••start_col ≥ 10<br>	∧ -1.reserved = {<br>	∧ -4.label in {<space>}<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, ,}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎⇥⁺<br>Confidence: 0.988. Support: 9867.` |
| 27 | `  •••start_col ≥ 10<br>	∧ -1.reserved = {<br>	∧ -4.label not in {<space>}<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ -5.label in {<space>}<br>	∧ +1.reserved not in {(, ,}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎⇥⁺<br>Confidence: 0.982. Support: 1317.` |
| 28 | `  •••start_col ≥ 10<br>	∧ -1.reserved = {<br>	∧ -4.label not in {<space>}<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved not in {(, ,}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER}<br>⇒ y = ⏎⏎⇥⁺<br>Confidence: 0.952. Support: 675.` |
| 29 | `  •••start_col ≤ 9<br>	∧ -1.reserved = {<br>	∧ +1.reserved not in {(, ,}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 934.` |
| 30 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.reserved = ,<br>	∧ -3.roles not in {CALL}<br>	∧ +1.reserved not in {(, ,, ;}<br>	∧ +5.reserved not in {)}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type = ExpressionStatement<br>⇒ y = "<br>Confidence: 0.927. Support: 592.` |
| 31 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.reserved not in {==}<br>	∧ +1.reserved not in {(, ,, ;}<br>	∧ +5.reserved not in {)}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.976. Support: 8118.` |
| 32 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.reserved not in {(, ;}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 3<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 6000.` |
| 33 | `  •••start_col ≤ 71<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {STRING}<br>	∧ +1.reserved not in {(, ,, ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type = ArrowFunctionExpression<br>⇒ y = "<br>Confidence: 0.935. Support: 895.` |
| 34 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {LITERAL, STRING}<br>	∧ -4.internal_type = BooleanLiteral<br>	∧ +1.reserved not in {(, ,, ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {ArrowFunctionExpression}<br>⇒ y = "<br>Confidence: 0.983. Support: 201.` |
| 35 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {LITERAL, STRING}<br>	∧ -3.internal_type = Identifier<br>	∧ +1.reserved not in {(, ,, ;}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.995. Support: 92.` |
| 36 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {LITERAL, STRING}<br>	∧ -3.internal_type not in {Identifier}<br>	∧ -3.reserved = ,<br>	∧ -4.internal_type not in {BooleanLiteral}<br>	∧ +1.reserved not in {(, ,, ;}<br>	∧ +2.reserved not in {;}<br>	∧ +5.reserved not in {)}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.933. Support: 547.` |
| 37 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {LITERAL, STRING}<br>	∧ -3.internal_type not in {Identifier}<br>	∧ -3.reserved not in {,}<br>	∧ -4.internal_type not in {BooleanLiteral}<br>	∧ +1.reserved not in {(, ,, ;}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.956. Support: 4456.` |
| 38 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {STRING} and not in {LITERAL}<br>	∧ +1.reserved not in {(, ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 601.` |
| 39 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {KEY} and not in {STRING}<br>	∧ +1.reserved not in {(, ;}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 5113.` |
| 40 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -2.label in {<newline>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎⇥⁻<br>Confidence: 0.983. Support: 2329.` |
| 41 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -2.label in {<-space>} and not in {<newline>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 146.` |
| 42 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ +1.reserved not in {(, }}<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 5<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 139.` |
| 43 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ +1.reserved not in {(, ,, }}<br>	∧ +1.roles not in {COMMENT, STRING}<br>	∧ +1.length ≥ 5<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ⏎⏎<br>Confidence: 0.979. Support: 4126.` |
| 44 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 4<br>	∧ +2.reserved = (<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 121.` |
| 45 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ +1.reserved not in {(, ), ,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 4<br>	∧ +2.reserved = (<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.977. Support: 950.` |
| 46 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ +1.reserved not in {(, }}<br>	∧ +1.roles in {EXPRESSION, VALUE}<br>	∧ +1.length ≤ 4<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 92.` |
| 47 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ +1.reserved not in {(, ,, }}<br>	∧ +1.roles in {EXPRESSION} and not in {COMMENT}<br>	∧ +1.length ≤ 4<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.957. Support: 403.` |
| 48 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved not in {(, ,, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 4<br>	∧ +2.internal_type not in {Identifier}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 3716.` |
| 49 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles in {COMMENT} and not in {KEY, STRING}<br>	∧ +1.reserved not in {(, ,, ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.internal_type = CommentLine<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.937. Support: 1138.` |
| 50 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles in {COMMENT} and not in {KEY, STRING}<br>	∧ -3.label in {<newline>}<br>	∧ +1.reserved not in {(, ,, ;}<br>	∧ +5.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.931. Support: 311.` |
| 51 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles in {COMMENT} and not in {KEY, STRING}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.reserved = }<br>	∧ +1.reserved not in {(, ,, ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎⏎<br>Confidence: 0.949. Support: 244.` |
| 52 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles in {COMMENT} and not in {STRING}<br>	∧ +1.reserved not in {(}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 831.` |
| 53 | `  -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ +1.reserved not in {(, ,, ;}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.967. Support: 537.` |
| 54 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {COMMENT, KEY, STRING}<br>	∧ -2.label in {<newline>}<br>	∧ +1.reserved not in {(, ,, ;}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 153.` |
| 55 | `  •••start_col ≤ 43<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<newline>}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved not in {(, ,, ;}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.936. Support: 2684.` |
| 56 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {COMMENT, STRING}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 5914.` |
| 57 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles in {EXPRESSION} and not in {COMMENT, STRING}<br>	∧ -2.reserved = (<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 464.` |
| 58 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {{, }}<br>	∧ -1.roles not in {COMMENT, EXPRESSION, KEY, STATEMENT, STRING}<br>	∧ -2.reserved = (<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 437.` |
| 59 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {COMMENT, STRING}<br>	∧ -2.internal_type = Identifier<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 148.` |
| 60 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {{, }}<br>	∧ -1.roles not in {COMMENT, KEY, STATEMENT, STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -3.label not in {<space>}<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type = FunctionExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 370.` |
| 61 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {{, }}<br>	∧ -1.roles not in {COMMENT, KEY, STATEMENT, STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -3.label not in {<space>}<br>	∧ -4.label in {<space>} and not in {<newline>}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 4917.` |
| 62 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {{, }}<br>	∧ -1.roles not in {COMMENT, KEY, STATEMENT, STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label in {<space>}<br>	∧ -3.label not in {<space>}<br>	∧ -4.label not in {<newline>, <space>}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 617.` |
| 63 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {{, }}<br>	∧ -1.roles not in {COMMENT, KEY, STATEMENT, STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<space>}<br>	∧ -3.label not in {<space>}<br>	∧ -4.label not in {<newline>, <space>}<br>	∧ -4.reserved = .<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 174.` |
| 64 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {COMMENT, KEY, STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<space>}<br>	∧ -3.label not in {<space>}<br>	∧ -4.label not in {<newline>, <space>}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 111.` |
| 65 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {{, }}<br>	∧ -1.roles not in {COMMENT, KEY, STATEMENT, STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -3.label not in {<space>}<br>	∧ -4.label not in {<newline>}<br>	∧ -4.roles in {ARGUMENT}<br>	∧ -5.label in {<space>}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 3188.` |
| 66 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles in {ARGUMENT} and not in {COMMENT, KEY, STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -3.label not in {<space>}<br>	∧ -4.label not in {<newline>}<br>	∧ -4.roles in {ARGUMENT}<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 107.` |
| 67 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {{, }}<br>	∧ -1.roles not in {ARGUMENT, COMMENT, KEY, STATEMENT, STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -3.label not in {<space>}<br>	∧ -4.label not in {<newline>}<br>	∧ -4.roles in {ARGUMENT}<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 152.` |
| 68 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {{, }}<br>	∧ -1.roles not in {COMMENT, KEY, STATEMENT, STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -3.label not in {<space>}<br>	∧ -4.label not in {<newline>}<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 21019.` |
| 69 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {COMMENT, STRING}<br>	∧ -3.roles in {CASE}<br>	∧ +1.reserved not in {(, ), ,, ;}<br>	∧ ^1.internal_type = SwitchStatement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.967. Support: 382.` |
| 70 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {COMMENT, STRING}<br>	∧ -2.roles in {IMPORT}<br>	∧ -3.label in {<newline>}<br>	∧ +1.reserved not in {(, ), ,, ;}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.992. Support: 648.` |
| 71 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {COMMENT, KEY, STATEMENT, STRING}<br>	∧ -2.roles in {IMPORT}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved not in {(, ), ,, ;}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 173.` |
| 72 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{, }}<br>	∧ -1.roles not in {COMMENT, STRING}<br>	∧ -5.label in {<newline>}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.947. Support: 616.` |
| 73 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{, }}<br>	∧ -1.roles in {EXPRESSION} and not in {COMMENT, KEY, STATEMENT, STRING}<br>	∧ -2.label in {<space>}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 1506.` |
| 74 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles in {COMMENT}<br>	∧ -4.label in {<space>}<br>	∧ +1.reserved not in {(, ), }}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.length ≥ 65<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 132.` |
| 75 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = ArrowFunctionExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 780.` |
| 76 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {COMMENT, KEY, STRING}<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = ArrowFunctionExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 1904.` |
| 77 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {(, ), }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 134<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 169.` |
| 78 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {COMMENT, KEY, STRING}<br>	∧ -2.reserved = )<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.length ≤ 6<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 793.` |
| 79 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {COMMENT, KEY, STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 16<br>	∧ -5.label in {<space>}<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 115.` |
| 80 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {COMMENT, KEY, STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 16<br>	∧ -5.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 172.` |
| 81 | `  •••start_col ≥ 56<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {COMMENT, KEY, STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 16<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.length ≤ 1<br>	∧ +4.reserved = ,<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>	∧ ^2.roles in {CALL}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 156.` |
| 82 | `  •••start_col ≥ 56<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {COMMENT, KEY, STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 16<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.length ≤ 1<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 1229.` |
| 83 | `  •••start_col ≤ 55<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {COMMENT, KEY, STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 16<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.length ≤ 1<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>	∧ ^2.roles in {INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 3987.` |
| 84 | `  •••start_col ≤ 55<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {COMMENT, KEY, STRING}<br>	∧ -2.diff_offset ≥ 5<br>	∧ -2.reserved not in {)}<br>	∧ -3.diff_col ≥ 10<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 16<br>	∧ -5.roles in {EXPRESSION}<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.length ≤ 1<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>	∧ ^2.roles not in {INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 136.` |
| 85 | `  •••start_col ≤ 55<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {COMMENT, KEY, STRING}<br>	∧ -2.diff_offset ≤ 4<br>	∧ -2.reserved not in {)}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 16<br>	∧ -5.roles in {EXPRESSION}<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.length ≤ 1<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>	∧ ^2.roles not in {INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 646.` |
| 86 | `  •••start_col ≤ 22<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {COMMENT, KEY, STRING}<br>	∧ -2.diff_offset ≤ 4<br>	∧ -2.reserved not in {)}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 16<br>	∧ -5.roles in {EXPRESSION}<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.length ≤ 1<br>	∧ ^1.roles in {LITERAL, VALUE} and not in {IDENTIFIER}<br>	∧ ^2.roles not in {INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 134.` |
| 87 | `  •••start_col ≤ 15<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {COMMENT, KEY, STRING}<br>	∧ -2.diff_offset ≤ 4<br>	∧ -2.reserved not in {)}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 16<br>	∧ -5.roles in {EXPRESSION}<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.length ≤ 1<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>	∧ ^2.roles not in {INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 377.` |
| 88 | `  •••start_col ≤ 55<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {COMMENT, KEY, STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 16<br>	∧ -5.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.length ≤ 1<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>	∧ ^2.roles not in {INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 1146.` |
| 89 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {COMMENT, KEY, STRING}<br>	∧ +1.reserved = :<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {EXPRESSION} and not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 718.` |
| 90 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = :<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {EXPRESSION, IDENTIFIER, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 267.` |
| 91 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -4.reserved = :<br>	∧ +1.reserved not in {(, ), }}<br>	∧ +1.roles in {FUNCTION} and not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 99.` |
| 92 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -2.reserved not in {)}<br>	∧ +1.reserved not in {(, ), ,}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 5<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 19061.` |
| 93 | `  •••start_col ≤ 10<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.reserved = if<br>	∧ +1.reserved not in {(, ), ,}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 4<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 1543.` |
| 94 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -2.diff_col ≥ 11<br>	∧ -2.reserved not in {)}<br>	∧ +1.reserved not in {(, ), ,}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 4<br>	∧ +5.roles in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 169.` |
| 95 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -2.reserved not in {)}<br>	∧ +1.reserved not in {(, ), ,}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 4<br>	∧ +5.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 3737.` |
| 96 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -2.reserved not in {)}<br>	∧ +1.reserved not in {(, ), ,}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 3<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 9162.` |
| 97 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.reserved = .<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 2701.` |
| 98 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.reserved = )<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 221.` |
| 99 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.reserved = )<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 3042.` |
| 100 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.label not in {<newline>}<br>	∧ -3.reserved not in {), .}<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {VARIABLE} and not in {IDENTIFIER, LITERAL}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 235.` |
| 101 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.label not in {<newline>}<br>	∧ -3.reserved not in {), .}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {VARIABLE} and not in {IDENTIFIER, LITERAL}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 537.` |
| 102 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.label not in {<newline>}<br>	∧ -3.reserved not in {), .}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 14930.` |
| 103 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -2.reserved not in {)}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.reserved not in {), .}<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 43112.` |
| 104 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -2.reserved = (<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {.}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 727.` |
| 105 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {,, ;, [, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {.}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {MULTIPLY} and not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 133.` |
| 106 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {[, {}<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {.}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 1945.` |
| 107 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {KEY, STRING}<br>	∧ -2.reserved not in {(, )}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {), .}<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 34633.` |
| 108 | `  •••start_col ≤ 4<br>	∧ -1.diff_col ≥ 4<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ +1.reserved not in {(, ), ,, ;}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.987. Support: 196.` |
| 109 | `  •••start_col ≤ 4<br>	∧ -1.diff_col ≤ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {COMMENT, KEY, STATEMENT, STRING}<br>	∧ +1.reserved not in {(, ), ,, ;}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.roles in {EXPRESSION} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 235.` |
| 110 | `  •••start_col ≤ 4<br>	∧ -1.diff_col ≤ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {COMMENT, KEY, STATEMENT, STRING}<br>	∧ +1.reserved not in {(, ), ,, ;}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +3.reserved = )<br>	∧ ^1.roles not in {EXPRESSION, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 101.` |
| 111 | `  •••start_col ≤ 3<br>	∧ -1.diff_col ≤ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {COMMENT, STRING}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {KEY}<br>	∧ +3.reserved not in {)}<br>	∧ ^1.roles not in {EXPRESSION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 671.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 12.81081081081081, "max_conf": 0.9998201727867126, "max_support": 60880, "min_conf": 0.9253979921340942, "min_support": 92, "num_rules": 111}}
```
</details>
