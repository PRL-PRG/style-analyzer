# Model report for file:///tmp/top-repos-quality-repos-acyjc8oy/create-snowpack-app.git HEAD 45660ff36399f50de2d60d4af5743c3c0b65bdad

### Dump

```json
{'created_at': '2021-08-29 21:49:53',
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
 'size': '21.3 kB',
 'tags': [],
 'uuid': 'dff7d79a-8726-43ae-9b64-37497e329f4c',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-acyjc8oy/create-snowpack-app.git 45660ff36399f50de2d60d4af5743c3c0b65bdad

# javascript
150 rules, avg.len. 9.4
## train
PPCR: 0.953920
### report
macro
{'f1-score': 0.5286175062845614,
 'precision': 0.5309819958893377,
 'recall': 0.5267668746250915,
 'support': 46806}
micro
{'f1-score': 0.917339657308892,
 'precision': 0.917339657308892,
 'recall': 0.917339657308892,
 'support': 46806}
weighted
{'f1-score': 0.9085574179336213,
 'precision': 0.9008634465910721,
 'recall': 0.917339657308892,
 'support': 46806}
### report_full
macro
{'f1-score': 0.5014002088245167,
 'precision': 0.5309819958893377,
 'recall': 0.4847826402304947,
 'support': 49067}
micro
{'f1-score': 0.8957057774347315,
 'precision': 0.917339657308892,
 'recall': 0.8750687835001121,
 'support': 49067}
weighted
{'f1-score': 0.8790210253236064,
 'precision': 0.8869443278587213,
 'recall': 0.8750687835001121,
 'support': 49067}
## test
PPCR: 0.917201
### report
macro
{'f1-score': 0.3263242250417329,
 'precision': 0.33581965789846224,
 'recall': 0.32059947542213196,
 'support': 1573}
micro
{'f1-score': 0.8150031786395423,
 'precision': 0.8150031786395423,
 'recall': 0.8150031786395423,
 'support': 1573}
weighted
{'f1-score': 0.7894494455089808,
 'precision': 0.7691174910694522,
 'recall': 0.8150031786395423,
 'support': 1573}
### report_full
macro
{'f1-score': 0.29705954457994155,
 'precision': 0.33581965789846224,
 'recall': 0.28011878364352427,
 'support': 1715}
micro
{'f1-score': 0.7798053527980534,
 'precision': 0.8150031786395423,
 'recall': 0.7475218658892129,
 'support': 1715}
weighted
{'f1-score': 0.7434225153160184,
 'precision': 0.7592670249756066,
 'recall': 0.7475218658892129,
 'support': 1715}
```

## javascript
### Summary
93 rules, avg.len. 9.2

| | |
|-|-|
|Min support|153|
|Max support|9580|
|Min confidence|0.9219771027565002|
|Max confidence|0.9997013211250305|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 4382.` |
| 2 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 2251.` |
| 3 | `  -1.reserved not in {;, {}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 1653.` |
| 4 | `  -1.reserved not in {;, {}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 1445.` |
| 5 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.980. Support: 738.` |
| 6 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = if<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 595.` |
| 7 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, if}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.984. Support: 289.` |
| 8 | `  •••start_col ≥ 17<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.length ≤ 22<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 9556.` |
| 9 | `  •••start_col ≤ 16<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.length ≤ 22<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 1699.` |
| 10 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.987. Support: 4339.` |
| 11 | `  -1.reserved not in {;, {}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 1455.` |
| 12 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 864.` |
| 13 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 6563.` |
| 14 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 2814.` |
| 15 | `  -1.diff_col ≤ 8<br>	∧ -1.diff_offset ≥ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {BODY} and not in {LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 452.` |
| 16 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {BODY, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 4075.` |
| 17 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 1419.` |
| 18 | `  -1.reserved not in {(, ;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 917.` |
| 19 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.977. Support: 678.` |
| 20 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.987. Support: 268.` |
| 21 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = else<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 161.` |
| 22 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 254.` |
| 23 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 5889.` |
| 24 | `  •••start_col ≥ 17<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.roles not in {COMMENT}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 9580.` |
| 25 | `  •••start_col ≤ 16<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.roles not in {COMMENT}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 1665.` |
| 26 | `  -1.reserved not in {;}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 1478.` |
| 27 | `  -1.reserved not in {;}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 961.` |
| 28 | `  -1.reserved not in {;}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 880.` |
| 29 | `  -1.reserved not in {;}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 6577.` |
| 30 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 2798.` |
| 31 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 355.` |
| 32 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 3457.` |
| 33 | `  -1.reserved not in {(, ;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 1409.` |
| 34 | `  -1.reserved not in {(, ;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 945.` |
| 35 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.985. Support: 684.` |
| 36 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.994. Support: 268.` |
| 37 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 240.` |
| 38 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION} and not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 325.` |
| 39 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION} and not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 301.` |
| 40 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT, EXPRESSION}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 3895.` |
| 41 | `  -1.reserved not in {;, {}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 1427.` |
| 42 | `  -1.reserved not in {;, {}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 917.` |
| 43 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 874.` |
| 44 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 6624.` |
| 45 | `  -1.diff_col ≤ 8<br>	∧ -1.diff_offset ≥ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {BODY} and not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 425.` |
| 46 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {BODY, IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 4122.` |
| 47 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 1438.` |
| 48 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = else<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 153.` |
| 49 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved not in {(, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {BLOCK}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 203.` |
| 50 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 332.` |
| 51 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION} and not in {BLOCK}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 306.` |
| 52 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 3798.` |
| 53 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 4374.` |
| 54 | `  •••start_line ≥ 254<br>	∧ -1.reserved = {<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.924. Support: 1159.` |
| 55 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 2274.` |
| 56 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 6120.` |
| 57 | `  -1.reserved not in {;, {}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1674.` |
| 58 | `  -1.reserved not in {;, {}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 1431.` |
| 59 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.985. Support: 722.` |
| 60 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = if<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 554.` |
| 61 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, if}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.995. Support: 294.` |
| 62 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = else<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 173.` |
| 63 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.length ≤ 22<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 6972.` |
| 64 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.length ≤ 22<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles in {EXPRESSION} and not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 313.` |
| 65 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>} and not in {<-space>}<br>	∧ -2.length ≤ 22<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles in {EXPRESSION} and not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 236.` |
| 66 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.length ≤ 22<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT, EXPRESSION}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 3521.` |
| 67 | `  •••start_line ≥ 240<br>	∧ -1.reserved = {<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.929. Support: 1146.` |
| 68 | `  -1.reserved not in {;, {}<br>	∧ -2.label in {<space>}<br>	∧ -3.internal_type = Identifier<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 893.` |
| 69 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {BODY} and not in {LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 454.` |
| 70 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved not in {(, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {BLOCK}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 208.` |
| 71 | `  -1.reserved not in {;, {}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 1443.` |
| 72 | `  -1.reserved not in {;, {}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 992.` |
| 73 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 845.` |
| 74 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 6520.` |
| 75 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 2845.` |
| 76 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {LITERAL, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 4640.` |
| 77 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 1385.` |
| 78 | `  -1.reserved not in {(, ;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 896.` |
| 79 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.984. Support: 719.` |
| 80 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.991. Support: 268.` |
| 81 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = else<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 173.` |
| 82 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 268.` |
| 83 | `  -1.diff_col ≤ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<-space>}<br>	∧ -4.label in {<newline>}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 220.` |
| 84 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<-space>}<br>	∧ -4.label not in {<newline>}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 4751.` |
| 85 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 6528.` |
| 86 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 2834.` |
| 87 | `  -1.diff_col ≤ 9<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 4681.` |
| 88 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION} and not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 335.` |
| 89 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION} and not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 293.` |
| 90 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {COMMENT, EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 3866.` |
| 91 | `  •••start_line ≥ 243<br>	∧ -1.reserved = {<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.943. Support: 1172.` |
| 92 | `  -1.reserved not in {;, {}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 921.` |
| 93 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 6568.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.150537634408602, "max_conf": 0.9997013211250305, "max_support": 9580, "min_conf": 0.9219771027565002, "min_support": 153, "num_rules": 93}}
```
</details>
