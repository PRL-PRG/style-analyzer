# Model report for file:///tmp/top-repos-quality-repos-vry1bsel/redux.git HEAD ca9463dd692ddcf85073bca921fd05f970bba6cf

### Dump

```json
{'created_at': '2021-08-18 13:54:50',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-80-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.9 (default, Jan 26 2021, 15:33:00) [GCC 8.4.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '19.1 kB',
 'tags': [],
 'uuid': 'dd0a676f-d352-42c6-a823-81b2ca0d7f2d',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-vry1bsel/redux.git ca9463dd692ddcf85073bca921fd05f970bba6cf

# javascript
167 rules, avg.len. 7.4
## train
PPCR: 0.977920
### report
macro
{'f1-score': 0.8326810743856864,
 'precision': 0.8942082478228982,
 'recall': 0.7878077108886365,
 'support': 22101}
micro
{'f1-score': 0.908239446178906,
 'precision': 0.908239446178906,
 'recall': 0.908239446178906,
 'support': 22101}
weighted
{'f1-score': 0.9054305253718413,
 'precision': 0.907284474528126,
 'recall': 0.908239446178906,
 'support': 22101}
### report_full
macro
{'f1-score': 0.7897961742613343,
 'precision': 0.8942082478228982,
 'recall': 0.7325741593484265,
 'support': 22600}
micro
{'f1-score': 0.8981007136305675,
 'precision': 0.908239446178906,
 'recall': 0.8881858407079646,
 'support': 22600}
weighted
{'f1-score': 0.8917804939948881,
 'precision': 0.9073606624946701,
 'recall': 0.8881858407079646,
 'support': 22600}
## test
PPCR: 0.988042
### report
macro
{'f1-score': 0.8249680331805254,
 'precision': 0.9097584921209366,
 'recall': 0.7711607538944415,
 'support': 5288}
micro
{'f1-score': 0.898071104387292,
 'precision': 0.898071104387292,
 'recall': 0.898071104387292,
 'support': 5288}
weighted
{'f1-score': 0.8936310991585548,
 'precision': 0.9014234120211826,
 'recall': 0.898071104387292,
 'support': 5288}
### report_full
macro
{'f1-score': 0.7986822496830012,
 'precision': 0.9097584921209366,
 'recall': 0.7381131652075604,
 'support': 5352}
micro
{'f1-score': 0.8926691729323308,
 'precision': 0.898071104387292,
 'recall': 0.8873318385650224,
 'support': 5352}
weighted
{'f1-score': 0.8860907098351156,
 'precision': 0.9010168512489692,
 'recall': 0.8873318385650224,
 'support': 5352}
```

## javascript
### Summary
167 rules, avg.len. 7.4

| | |
|-|-|
|Min support|133|
|Max support|3456|
|Min confidence|0.805084764957428|
|Max confidence|0.9993864893913269|

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
               'min_samples_leaf_max': 130,
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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.999. Support: 815.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.988. Support: 2322.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.999. Support: 492.` |
| 4 | `  -1.diff_col ≤ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.915. Support: 1356.` |
| 5 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.999. Support: 427.` |
| 6 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = '<br>Confidence: 0.988. Support: 210.` |
| 7 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 466.` |
| 8 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≤ 3<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.840. Support: 435.` |
| 9 | `  •••start_col ≥ 8<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.823. Support: 466.` |
| 10 | `  •••start_col ≥ 8<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {BLOCK, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.892. Support: 1144.` |
| 11 | `  •••start_col ≤ 7<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.label in {<-space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ⏎⏎<br>Confidence: 0.926. Support: 182.` |
| 12 | `  •••start_col ≤ 7<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.length ≥ 2<br>	∧ +5.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 163.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.870. Support: 142.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 707.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 185.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.913. Support: 178.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.872. Support: 549.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 179.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {MAP}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 407.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 166.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.879. Support: 169.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +3.length ≥ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 2443.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +3.length ≥ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.831. Support: 2165.` |
| 24 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = '<br>Confidence: 0.993. Support: 230.` |
| 25 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 465.` |
| 26 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ -3.length ≥ 2<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.864. Support: 166.` |
| 27 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.length ≤ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.848. Support: 502.` |
| 28 | `  •••start_col ≥ 8<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {BLOCK, LITERAL}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ␣<br>Confidence: 0.840. Support: 1318.` |
| 29 | `  •••start_col ≤ 4<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.887. Support: 181.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 137.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 666.` |
| 32 | `  •••start_col ≤ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {MAP}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.809. Support: 233.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.892. Support: 171.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles in {KEY}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.863. Support: 157.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.roles not in {KEY}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +3.length ≥ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 2400.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {KEY}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +3.length ≥ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.813. Support: 2081.` |
| 37 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 2419.` |
| 38 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 786.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 483.` |
| 40 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.903. Support: 1310.` |
| 41 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 428.` |
| 42 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 235.` |
| 43 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ -3.length ≥ 2<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.828. Support: 183.` |
| 44 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.length ≤ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.826. Support: 491.` |
| 45 | `  •••start_col ≥ 8<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.815. Support: 511.` |
| 46 | `  •••start_col ≥ 8<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement, JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.903. Support: 1127.` |
| 47 | `  •••start_col ≥ 8<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = import<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 5<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.950. Support: 191.` |
| 48 | `  •••start_col ≤ 7<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.label in {<-space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.928. Support: 160.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 148.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 505.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, >}<br>	∧ +1.length ≤ 1<br>	∧ +3.roles in {EXPRESSION}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.889. Support: 312.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, >}<br>	∧ +1.length ≤ 1<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.854. Support: 175.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.903. Support: 159.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.873. Support: 571.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 2521.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.919. Support: 648.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.908. Support: 656.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 296.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.812. Support: 199.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ><br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 148.` |
| 61 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 2415.` |
| 62 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 813.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 470.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 444.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.989. Support: 232.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 461.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≤ 3<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.832. Support: 468.` |
| 68 | `  •••start_col ≥ 33<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {EXPRESSION} and not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.906. Support: 154.` |
| 69 | `  •••start_col ≥ 33<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {EXPRESSION, LITERAL, QUALIFIED, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.816. Support: 133.` |
| 70 | `  •••start_col ≤ 32<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement, JSXElement}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.915. Support: 1612.` |
| 71 | `  •••start_col ≤ 32<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.length ≤ 2<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement, JSXElement}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.878. Support: 579.` |
| 72 | `  •••start_col ≤ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.label in {<-space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.931. Support: 166.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 157.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 530.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, >}<br>	∧ +1.length ≤ 1<br>	∧ +3.roles in {EXPRESSION}<br>	∧ ^1.roles in {DECLARATION} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 308.` |
| 76 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved not in {=, >}<br>	∧ +1.length ≤ 1<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles in {DECLARATION} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.835. Support: 179.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.841. Support: 167.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.839. Support: 549.` |
| 79 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.919. Support: 2548.` |
| 80 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 634.` |
| 81 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, QUALIFIED}<br>	∧ ^2.internal_type = ExpressionStatement<br>⇒ y = ␣<br>Confidence: 0.990. Support: 446.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, QUALIFIED}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 211.` |
| 83 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, :}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, QUALIFIED}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>⇒ y = ␣<br>Confidence: 0.896. Support: 226.` |
| 84 | `  -1.diff_col ≤ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.914. Support: 1358.` |
| 85 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 419.` |
| 86 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 244.` |
| 87 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ -3.length ≥ 2<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.823. Support: 150.` |
| 88 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.length ≤ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.821. Support: 483.` |
| 89 | `  •••start_col ≥ 8<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.816. Support: 422.` |
| 90 | `  •••start_col ≥ 8<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {BLOCK, LITERAL, QUALIFIED}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ␣<br>Confidence: 0.840. Support: 1360.` |
| 91 | `  •••start_col ≤ 7<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.label in {<-space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.944. Support: 170.` |
| 92 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.reserved not in {=, >}<br>	∧ +1.length ≤ 1<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles in {DECLARATION} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.844. Support: 196.` |
| 93 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 2352.` |
| 94 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≤ 3<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.842. Support: 484.` |
| 95 | `  •••start_col ≥ 8<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {BODY, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.903. Support: 1093.` |
| 96 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 172.` |
| 97 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.internal_type = JSXOpeningElement<br>⇒ y = ∅<br>Confidence: 0.889. Support: 149.` |
| 98 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 649.` |
| 99 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.893. Support: 201.` |
| 100 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {MAP}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 379.` |
| 101 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 169.` |
| 102 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.880. Support: 171.` |
| 103 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +3.length ≥ 1<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 2419.` |
| 104 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.805. Support: 177.` |
| 105 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +3.length ≥ 1<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.835. Support: 2110.` |
| 106 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.989. Support: 229.` |
| 107 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 469.` |
| 108 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ -3.length ≥ 2<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.808. Support: 164.` |
| 109 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -3.length ≤ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.862. Support: 466.` |
| 110 | `  •••start_col ≥ 19<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.842. Support: 396.` |
| 111 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.829. Support: 2946.` |
| 112 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≥ 1<br>⇒ y = ∅<br>Confidence: 0.971. Support: 2471.` |
| 113 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≥ 1<br>⇒ y = ∅<br>Confidence: 0.819. Support: 2117.` |
| 114 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 527.` |
| 115 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {EXPRESSION, STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.983. Support: 267.` |
| 116 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 506.` |
| 117 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {INCOMPLETE} and not in {IDENTIFIER, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.872. Support: 145.` |
| 118 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -4.diff_col ≥ 13<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.831. Support: 169.` |
| 119 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.871. Support: 2295.` |
| 120 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 161.` |
| 121 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {DECLARATION, FUNCTION} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 160.` |
| 122 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, >}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.843. Support: 150.` |
| 123 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {>}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.891. Support: 999.` |
| 124 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.898. Support: 162.` |
| 125 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.871. Support: 561.` |
| 126 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.915. Support: 193.` |
| 127 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.816. Support: 633.` |
| 128 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 396.` |
| 129 | `  •••start_col ≥ 25<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = import<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 170.` |
| 130 | `  •••start_col ≤ 24<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.887. Support: 200.` |
| 131 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ><br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.874. Support: 218.` |
| 132 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -2.label not in {'}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ +5.length ≥ 1<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 2164.` |
| 133 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {'}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ +5.length ≥ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, IDENTIFIER}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 167.` |
| 134 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {'}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ +5.length ≥ 1<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.876. Support: 1321.` |
| 135 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.998. Support: 302.` |
| 136 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 158.` |
| 137 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {>}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.853. Support: 146.` |
| 138 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {>}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.892. Support: 2204.` |
| 139 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.917. Support: 3456.` |
| 140 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 1230.` |
| 141 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 1132.` |
| 142 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.993. Support: 224.` |
| 143 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 925.` |
| 144 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.885. Support: 414.` |
| 145 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type = JSXOpeningElement<br>⇒ y = ∅<br>Confidence: 0.942. Support: 405.` |
| 146 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.871. Support: 345.` |
| 147 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles in {LITERAL} and not in {DECLARATION}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ⏎<br>Confidence: 0.863. Support: 531.` |
| 148 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {LITERAL} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.847. Support: 154.` |
| 149 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ><br>	∧ ^1.roles not in {DECLARATION, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 200.` |
| 150 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), >, }}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, LITERAL}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = '<br>Confidence: 0.997. Support: 176.` |
| 151 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_col ≤ 10<br>	∧ +1.reserved not in {), >, }}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, LITERAL}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.841. Support: 350.` |
| 152 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {', (, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), >, }}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, FILE, LITERAL, SCOPE}<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ␣<br>Confidence: 0.845. Support: 2719.` |
| 153 | `  -1.diff_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.911. Support: 1320.` |
| 154 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.989. Support: 227.` |
| 155 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {}}<br>	∧ +4.reserved not in {}}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.859. Support: 549.` |
| 156 | `  •••start_col ≥ 8<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement, JSXElement}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.898. Support: 1132.` |
| 157 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.808. Support: 159.` |
| 158 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 646.` |
| 159 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.849. Support: 202.` |
| 160 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION} and not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 169.` |
| 161 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 2601.` |
| 162 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.923. Support: 356.` |
| 163 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 150.` |
| 164 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.884. Support: 176.` |
| 165 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 653.` |
| 166 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 165.` |
| 167 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.833. Support: 2001.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.377245508982036, "max_conf": 0.9993864893913269, "max_support": 3456, "min_conf": 0.805084764957428, "min_support": 133, "num_rules": 167}}
```
</details>
