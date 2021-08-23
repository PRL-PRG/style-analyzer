# Model report for file:///tmp/top-repos-quality-repos-86wcyod8/django_local_library.git HEAD 2f8e44a0395657712ec3fb621cf20ddf3a55be19

### Dump

```json
{'created_at': '2021-08-21 20:29:46',
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
 'size': '18.7 kB',
 'tags': [],
 'uuid': '7f34c4d5-68ab-4f14-9dbf-4450078f9d9c',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-86wcyod8/django_local_library.git 2f8e44a0395657712ec3fb621cf20ddf3a55be19

# javascript
119 rules, avg.len. 7.3
## train
PPCR: 0.952300
### report
macro
{'f1-score': 0.672694890690626,
 'precision': 0.6915268297755646,
 'recall': 0.6590313906102576,
 'support': 14494}
micro
{'f1-score': 0.9177590727197461,
 'precision': 0.9177590727197461,
 'recall': 0.9177590727197461,
 'support': 14494}
weighted
{'f1-score': 0.9055899645176814,
 'precision': 0.8952192481235636,
 'recall': 0.9177590727197461,
 'support': 14494}
### report_full
macro
{'f1-score': 0.650730383428144,
 'precision': 0.6915268297755646,
 'recall': 0.6199783070397784,
 'support': 15220}
micro
{'f1-score': 0.8953355320724237,
 'precision': 0.9177590727197461,
 'recall': 0.8739816031537451,
 'support': 15220}
weighted
{'f1-score': 0.8800932390141586,
 'precision': 0.8890211608766859,
 'recall': 0.8739816031537451,
 'support': 15220}
## test
PPCR: 0.956174
### report
macro
{'f1-score': 0.6430948504629803,
 'precision': 0.6515007149609959,
 'recall': 0.6368165550291733,
 'support': 2269}
micro
{'f1-score': 0.9237549581313353,
 'precision': 0.9237549581313353,
 'recall': 0.9237549581313353,
 'support': 2269}
weighted
{'f1-score': 0.9183116573522525,
 'precision': 0.9137507673001782,
 'recall': 0.9237549581313353,
 'support': 2269}
### report_full
macro
{'f1-score': 0.618541942270812,
 'precision': 0.6515007149609959,
 'recall': 0.592228214792534,
 'support': 2373}
micro
{'f1-score': 0.903059026281775,
 'precision': 0.9237549581313353,
 'recall': 0.8832701222081754,
 'support': 2373}
weighted
{'f1-score': 0.8956016197818186,
 'precision': 0.9124113069378545,
 'recall': 0.8832701222081754,
 'support': 2373}
```

## javascript
### Summary
79 rules, avg.len. 7.3

| | |
|-|-|
|Min support|136|
|Max support|4607|
|Min confidence|0.9203730225563049|
|Max confidence|0.9992101192474365|

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
                     'min_samples_split': 181,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.label in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.933. Support: 811.` |
| 2 | `  -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 409.` |
| 3 | `  -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.989. Support: 224.` |
| 4 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.987. Support: 267.` |
| 5 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 302.` |
| 6 | `  -1.reserved = ,<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 148.` |
| 7 | `  -1.reserved = var<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 169.` |
| 8 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.959. Support: 159.` |
| 9 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;, var, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -4.diff_line = 0<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 4127.` |
| 10 | `  -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1880.` |
| 11 | `  -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.990. Support: 240.` |
| 12 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.982. Support: 304.` |
| 13 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 451.` |
| 14 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 238.` |
| 15 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 1671.` |
| 16 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 636.` |
| 17 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 467.` |
| 18 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT} and not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 172.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 437.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 226.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.989. Support: 219.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {FUNCTION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 136.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 186.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = var<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 160.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.971. Support: 153.` |
| 26 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.955. Support: 146.` |
| 27 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, var, {, }}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 727.` |
| 28 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, var, {, }}<br>	∧ -4.diff_line = 0<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 3870.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 309.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.989. Support: 226.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {INITIALIZATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 165.` |
| 32 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.946. Support: 175.` |
| 33 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = var<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 164.` |
| 34 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ -3.diff_line = 0<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 4607.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 1908.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.978. Support: 204.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 468.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 404.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 1621.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.969. Support: 239.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 661.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 492.` |
| 43 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.943. Support: 184.` |
| 44 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT} and not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 179.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.982. Support: 247.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 424.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.975. Support: 261.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 286.` |
| 49 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, var, {, }}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 634.` |
| 50 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, var, {, }}<br>	∧ -3.diff_line = 0<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 4524.` |
| 51 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.982. Support: 251.` |
| 52 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 275.` |
| 53 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.944. Support: 151.` |
| 54 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;, var, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 736.` |
| 55 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;, var, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -4.diff_line = 0<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 4120.` |
| 56 | `  -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.978. Support: 203.` |
| 57 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 421.` |
| 58 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_line = 0<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 4511.` |
| 59 | `  -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 1926.` |
| 60 | `  -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.985. Support: 229.` |
| 61 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 448.` |
| 62 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.975. Support: 255.` |
| 63 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 389.` |
| 64 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 1630.` |
| 65 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 685.` |
| 66 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 468.` |
| 67 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT} and not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 182.` |
| 68 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT, LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.950. Support: 150.` |
| 69 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.length ≥ 2<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT, LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {LITERAL, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 697.` |
| 70 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.length ≤ 1<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {COMMENT, LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {LITERAL, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 139.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.993. Support: 1891.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.988. Support: 213.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 456.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.986. Support: 251.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 388.` |
| 76 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 1682.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 633.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 485.` |
| 79 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT} and not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 161.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.341772151898734, "max_conf": 0.9992101192474365, "max_support": 4607, "min_conf": 0.9203730225563049, "min_support": 136, "num_rules": 79}}
```
</details>
