# Model report for file:///tmp/top-repos-quality-repos-bndkwo4g/sights.git HEAD e9269a3f5c392e33160b1d036c3d979089a9b31d

### Dump

```json
{'created_at': '2021-08-20 19:29:17',
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
 'size': '20.7 kB',
 'tags': [],
 'uuid': 'f97f066a-68c7-49a1-bade-f8d54cb75a1f',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-bndkwo4g/sights.git e9269a3f5c392e33160b1d036c3d979089a9b31d

# javascript
109 rules, avg.len. 7.0
## train
PPCR: 0.950440
### report
macro
{'f1-score': 0.588028865340612,
 'precision': 0.6189238577921969,
 'recall': 0.5707812816997361,
 'support': 16301}
micro
{'f1-score': 0.9073063002269799,
 'precision': 0.9073063002269799,
 'recall': 0.9073063002269799,
 'support': 16301}
weighted
{'f1-score': 0.8955615048498141,
 'precision': 0.8889256235651029,
 'recall': 0.9073063002269799,
 'support': 16301}
### report_full
macro
{'f1-score': 0.5591387138372351,
 'precision': 0.6189238577921969,
 'recall': 0.5331357918742367,
 'support': 17151}
micro
{'f1-score': 0.8842520626569413,
 'precision': 0.9073063002269799,
 'recall': 0.8623403883155502,
 'support': 17151}
weighted
{'f1-score': 0.8667410107236593,
 'precision': 0.8850093197836447,
 'recall': 0.8623403883155502,
 'support': 17151}
## test
PPCR: 0.924042
### report
macro
{'f1-score': 0.3945706945540896,
 'precision': 0.3872495057852652,
 'recall': 0.41405475540324915,
 'support': 4197}
micro
{'f1-score': 0.8036692875863712,
 'precision': 0.8036692875863712,
 'recall': 0.8036692875863712,
 'support': 4197}
weighted
{'f1-score': 0.769047308905674,
 'precision': 0.749094541493957,
 'recall': 0.8036692875863712,
 'support': 4197}
### report_full
macro
{'f1-score': 0.3810452822320442,
 'precision': 0.3872495057852652,
 'recall': 0.38588090471375835,
 'support': 4542}
micro
{'f1-score': 0.7719418697791509,
 'precision': 0.8036692875863712,
 'recall': 0.7426243945398503,
 'support': 4542}
weighted
{'f1-score': 0.7283530751363706,
 'precision': 0.7283555825038557,
 'recall': 0.7426243945398503,
 'support': 4542}
```

## javascript
### Summary
64 rules, avg.len. 6.9

| | |
|-|-|
|Min support|142|
|Max support|3693|
|Min confidence|0.9200913310050964|
|Max confidence|0.9995126724243164|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles in {MAP}<br>⇒ y = "<br>Confidence: 0.988. Support: 1746.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.reserved = "<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 643.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_col ≥ 4<br>	∧ -4.reserved = "<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 652.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_col ≤ 4<br>	∧ -4.reserved = "<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.985. Support: 291.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label not in {<space>}<br>	∧ -4.reserved not in {"}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {MAP}<br>⇒ y = "<br>Confidence: 0.974. Support: 1730.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 436.` |
| 7 | `  -1.diff_col ≤ 16<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 2059.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.label in {"}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 689.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_col ≥ 4<br>	∧ -4.label in {"}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 712.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_col ≤ 4<br>	∧ -4.label in {"}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.982. Support: 308.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label not in {<space>}<br>	∧ -4.label not in {"}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {MAP}<br>⇒ y = "<br>Confidence: 0.969. Support: 1813.` |
| 12 | `  •••start_col ≥ 13<br>	∧ -1.diff_col ≤ 19<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 3268.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.990. Support: 1414.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 462.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 200.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, :, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 151.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 2179.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -4.label not in {"}<br>	∧ -5.diff_offset ≥ 22<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.928. Support: 1863.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 441.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.968. Support: 268.` |
| 21 | `  •••start_col ≥ 13<br>	∧ -1.diff_col ≤ 18<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 3355.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -4.reserved = "<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 698.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -4.reserved = "<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.991. Support: 284.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 1448.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles in {EXPRESSION} and not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.977. Support: 277.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 446.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 219.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, ;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 1061.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 342.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 253.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 206.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.diff_col ≥ 5<br>	∧ -4.label in {"}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 681.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.diff_col ≤ 5<br>	∧ -4.label in {"}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.972. Support: 308.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 169.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {VARIABLE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 154.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {CALLEE}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 1154.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {CALLEE}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 344.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {CALLEE}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 268.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {CALLEE}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 196.` |
| 40 | `  -1.reserved = :<br>	∧ -4.label in {"}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1026.` |
| 41 | `  -1.reserved = ,<br>	∧ -4.label in {"}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.995. Support: 692.` |
| 42 | `  -1.reserved not in {,, :}<br>	∧ -4.label in {"}<br>	∧ +1.length ≥ 4<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.998. Support: 243.` |
| 43 | `  -1.reserved = ,<br>	∧ -2.label not in {<-space>}<br>	∧ -4.label not in {"}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.929. Support: 163.` |
| 44 | `  -1.reserved not in {,}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -4.label not in {"}<br>	∧ ^1.roles in {MAP}<br>⇒ y = "<br>Confidence: 0.933. Support: 3693.` |
| 45 | `  +1.reserved not in {)}<br>	∧ ^1.roles in {QUALIFIED} and not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 1372.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {MAP, OPERATOR, QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 231.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {MAP, OPERATOR, QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 976.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.reserved not in {"}<br>	∧ -5.diff_offset ≥ 21<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.930. Support: 1938.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 436.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.920. Support: 219.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, ;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 1190.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.diff_col ≥ 5<br>	∧ -4.reserved = "<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 701.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.diff_col ≤ 5<br>	∧ -4.reserved = "<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.984. Support: 282.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.reserved not in {"}<br>	∧ -5.diff_offset ≥ 22<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.928. Support: 1946.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, ;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 1125.` |
| 56 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {:, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ⏎<br>Confidence: 0.969. Support: 145.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.diff_offset ≥ 5<br>	∧ -4.reserved = "<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 665.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.diff_offset ≤ 5<br>	∧ -4.reserved = "<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.981. Support: 284.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles in {INITIALIZATION} and not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.998. Support: 232.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 142.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 1071.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 378.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 282.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 231.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.859375, "max_conf": 0.9995126724243164, "max_support": 3693, "min_conf": 0.9200913310050964, "min_support": 142, "num_rules": 64}}
```
</details>
